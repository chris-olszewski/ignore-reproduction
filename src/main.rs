use ignore::{WalkBuilder, Walk};

fn main() {
    let walker_1 = WalkBuilder::new("./output/")
            .hidden(false)
            .git_ignore(true)
            .git_global(false)
            .git_exclude(true)
            .build();

   let walker_2 = WalkBuilder::new("./")
            .hidden(false)
            .git_ignore(true)
            .git_global(false)
            .git_exclude(true)
            .filter_entry(|entry| entry.path().starts_with("./output"))
            .build();
 
   println!("Walker from subdir");
   show_entry(walker_1);
   println!("Walker from root");
   show_entry(walker_2);
}

fn show_entry(walker: Walk) {
    for result in walker {
	match result {
    		Ok(entry) => println!("{}", entry.path().display()),
    		Err(err) => println!("err: {err}"),
	}
    }
}
