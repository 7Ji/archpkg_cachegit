use std::{env::args, fs::{create_dir, File}, io::Write, path::Path, process::Command};

use pkgbuild::{GitSourceFragment, SourceProtocol};

use url::Url;

fn main() {
    let gmr = args().nth(1).expect("Prefix of git-mirrorer must be declared as the first argument");
    let pkgbuild = pkgbuild::parse_one(Some("PKGBUILD")).unwrap();
    for source in pkgbuild.sources.iter() {
        let fetchspec = 
            if let SourceProtocol::Git { fragment, signed: _ } = &source.protocol {
                if let Some(fragment) = fragment {
                    match fragment {
                        GitSourceFragment::Branch(branch) => format!("heads/{}", branch),
                        GitSourceFragment::Commit(_) => "*".into(),
                        GitSourceFragment::Tag(tag) => format!("tags/{}", tag),
                    }
                } else {
                    "*".into()
                }
            } else {
                continue
            };
        let repo = Path::new(&source.name);
        if repo.exists() {
            if ! repo.is_dir() {
                eprintln!("{} exists and is not a dir", &source.name);
                panic!("Path of git repo occupied by non-dir file");
            }
        } else {
            create_dir(repo).expect("Failed to create git repo dir");
            for suffix in ["objects", "refs"] {
                create_dir(repo.join(suffix)).expect("Failed to create git repo subdir");
            }
            let mut head = File::create(repo.join("HEAD")).expect("Failed to create HEAD");
            head.write_all("ref: refs/heads/*\n".as_bytes()).expect("Failed to write to HEAD");
            let mut config = File::create(repo.join("config")).expect("Failed to create config");
            write!(config, "\
                [core]\n\
                \trepositoryformatversion = 0\n\
                \tfilemode = true\n\
                \tbare = true\n\
                [remote \"origin\"]\n\
                \turl = {}\n\
                \tfetch = +refs/*:refs/*\n",
                source.url
            ).expect("Failed to write to git config");
        }
        let url = Url::parse(&source.url).expect("Failed to parse git source url");
        let mut gmr_url = gmr.clone();
        if let Some(domain) = url.domain() {
            gmr_url.push('/');
            gmr_url.push_str(domain);
        }
        gmr_url.push_str(url.path());
        println!("Caching git source '{}' from gmr '{}'", source.name, &gmr_url);
        if ! Command::new("git")
            .arg("--git-dir")
            .arg(repo)
            .arg("fetch")
            .arg(&gmr_url)
            .arg(format!("+refs/{}:refs/{}", fetchspec, fetchspec))
            .spawn()
            .expect("Failed to spawn git process")
            .wait()
            .expect("Failed to wait for git process")
            .success() 
        {
            panic!("Git process failed")
        }
        
    }
}
