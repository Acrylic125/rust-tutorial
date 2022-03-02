use std::time::SystemTime;

pub fn try_traits() {
    let blog = Blog {
        title: String::from(""),
        author: String::from(""),
        release: 100,
    };
    let news = News {
        title: String::from(""),
        author: String::from(""),
        organization: String::from(""),
        release: 100,
    };
    print_age(&blog);
    print_age(&news);
}

pub fn print_age(article: &impl Article) {
    println!("{}", article.get_age());
}

pub trait Article {
    fn get_publisher(&self) -> String;
    fn get_published_timestamp(&self) -> u64;
    fn get_age(&self) -> u64 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Error getting time!")
            .as_secs()
            - self.get_published_timestamp()
    }
}

pub struct Blog {
    pub title: String,
    pub author: String,
    pub release: u64,
}

impl Article for Blog {
    fn get_publisher(&self) -> String {
        self.author.clone()
    }
    fn get_published_timestamp(&self) -> u64 {
        self.release
    }
}

pub struct News {
    pub title: String,
    pub author: String,
    pub organization: String,
    pub release: u64,
}

impl Article for News {
    fn get_publisher(&self) -> String {
        self.author.clone()
    }
    fn get_published_timestamp(&self) -> u64 {
        self.release
    }
}
