pub use command::S as Command;

mod command {
    use linearf::{item::*, source::*};
    use std::{ffi::OsString, path::PathBuf, process::Stdio};
    use tokio::{
        io::{AsyncBufReadExt, BufReader},
        process::Command
    };
    use tokio_stream::{wrappers::LinesStream, StreamExt};

    pub struct S<L> {
        linearf: Weak<L>
    }

    #[derive(Deserialize, Serialize, PartialEq)]
    pub struct P {
        dir: PathBuf,
        command: OsString,
        args: Vec<OsString>,
        #[serde(default)]
        without_query: bool
    }

    impl SourceParams for P {}

    impl<L> IsSource for S<L> {
        type Params = P;
    }

    impl<L> NewSource<L> for S<L>
    where
        L: linearf::Linearf + Send + Sync + 'static
    {
        fn new(linearf: Weak<L>) -> Source<<Self as IsSource>::Params>
        where
            Self: Sized
        {
            Source::from_simple(Self { linearf })
        }
    }

    impl<L> SimpleGenerator for S<L> {
        fn stream(
            &self,
            (vars, params): (&Arc<Vars>, &Arc<Self::Params>)
        ) -> Pin<Box<dyn Stream<Item = Item> + Send + Sync>> {
            let q = OsString::from(vars.query.clone());
            let args = |mut c: Command| {
                if params.without_query {
                    c.args(params.args.iter());
                } else {
                    c.args(params.args.iter().chain(Some(&q).into_iter()));
                }
                c
            };
            let r = args(Command::new(&params.command))
                .stdout(Stdio::piped())
                .spawn();
            let out = match r {
                Ok(tokio::process::Child {
                    stdout: Some(out), ..
                }) => out,
                _ => return Box::pin(empty())
            };
            let rdr = BufReader::new(out);
            let lines = LinesStream::new(rdr.lines());
            let s = lines.map(|s| todo!());
            Box::pin(s)
        }

        fn reusable(
            &self,
            (p, prev): (&Arc<Vars>, &Arc<Self::Params>),
            (v, params): (&Arc<Vars>, &Arc<Self::Params>)
        ) -> Reusable {
            if prev == params && (params.without_query || p.query == v.query) {
                Reusable::Cache
            } else {
                Reusable::None
            }
        }
    }
}
