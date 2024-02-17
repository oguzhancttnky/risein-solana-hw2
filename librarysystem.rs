enum Publication {
    Book(BookDetails),
    Magazine(MagazineDetails),
}

struct BookDetails {
    title: String,
    author: String,
    page_count: u32,
}

struct MagazineDetails {
    title: String,
    issue: u32,
    topic: String,
}

fn print_publication(publication: &Publication) {
    match publication {
        Publication::Book(book) => {
            println!("Kitap: {} yazar: {}, {} sayfa", book.title, book.author, book.page_count);
        }
        Publication::Magazine(magazine) => {
            println!("Dergi: {} - Sayı: {}, Konu: {}", magazine.title, magazine.issue, magazine.topic);
        }
    }
}

fn main() {
    let kitap1 = BookDetails {
        title: "Blindness".to_string(),
        author: "Jose Saramago".to_string(),
        page_count: 320,
    };
    
    let dergi1 = MagazineDetails {
        title: "ARKA KAPI".to_string(),
        issue: 12,
        topic: "Cyber Security".to_string(),
    };
    
    let mut publications: Vec<Publication> = Vec::new();
    publications.push(Publication::Book(kitap1));
    publications.push(Publication::Magazine(dergi1));
    
    for publication in publications.iter() {
        print_publication(publication);
    }
}
