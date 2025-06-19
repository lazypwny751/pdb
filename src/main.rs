mod libpdb;

use std::io;
use clap::Parser;
use libpdb::kvstore::KvStore;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
	#[arg(short, long)]
	get: Option<String>,

	#[arg(short, long)]
	set: Option<String>,

    file: String
}

fn main() -> io::Result<()> {
	let args = Args::parse();

	#[allow(unused_mut, unused_variables)]
	let mut db = KvStore::new(args.file);

	Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_get_delete() {
        let mut db = KvStore::new("test.pdb".to_string());

        db.set("foo".to_string(), "bar".to_string());
        db.set("baz".to_string(), "qux".to_string());

        assert_eq!(db.get("foo"), Some(&"bar".to_string()));
        assert_eq!(db.get("baz"), Some(&"qux".to_string()));

        let deleted = db.delete("foo");
        assert_eq!(deleted, Some("bar".to_string()));
        assert_eq!(db.get("foo"), None);
    }
}
