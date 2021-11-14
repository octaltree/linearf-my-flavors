pub use command::S as Command;

mod command {
    use futures::stream::StreamExt;
    use linearf::{item::*, source::*};
    use std::{ffi::OsStr, path::PathBuf, process::Stdio};
    use tokio::{
        io::{AsyncBufReadExt, BufReader},
        process::Command
    };
    use tokio_stream::wrappers::LinesStream;

    pub struct S<L> {
        _linearf: Weak<L>
    }

    #[derive(Deserialize, Serialize, PartialEq)]
    #[serde(untagged)]
    pub enum D {
        Utf8(String),
        P(PathBuf)
    }

    #[derive(Deserialize, Serialize, PartialEq)]
    pub struct P {
        dir: PathBuf,
        command: PathBuf,
        args: Vec<D>,
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
            Source::from_simple(Self { _linearf: linearf })
        }
    }

    impl AsRef<OsStr> for &D {
        fn as_ref(&self) -> &OsStr {
            match self {
                D::Utf8(s) => s.as_ref(),
                D::P(p) => p.as_ref()
            }
        }
    }

    impl<L> SimpleGenerator for S<L> {
        fn stream(
            &self,
            (vars, params): (&Arc<Vars>, &Arc<Self::Params>)
        ) -> Pin<Box<dyn Stream<Item = Item> + Send + Sync>> {
            let q = D::Utf8(vars.query.clone());
            let args = |mut c: Command| {
                if params.without_query {
                    c.args(params.args.iter());
                } else {
                    c.args(params.args.iter().chain(Some(&q).into_iter()));
                }
                c
            };
            let r = args(Command::new(&params.command))
                .current_dir(&params.dir)
                .stdout(Stdio::piped())
                .stderr(Stdio::null())
                .spawn();
            let out = match r {
                Ok(tokio::process::Child {
                    stdout: Some(out), ..
                }) => out,
                _ => return Box::pin(empty())
            };
            let rdr = BufReader::new(out);
            let lines = LinesStream::new(rdr.lines());
            let s = lines
                .filter_map(|r| async { r.ok() })
                .enumerate()
                .map(|(i, s)| {
                    let i: u32 = i.try_into().unwrap();
                    Item::new(i + 1, MaybeUtf8::Utf8(s))
                });
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
