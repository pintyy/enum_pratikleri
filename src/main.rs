#[derive(Debug)]
enum Media {
    Book {title:String, author:String},
    Movie {title:String, director:String},
    Audiobook {title:String},
    Stringlerin_arasına_int_denemesi(u32),
    Boş_metin
}

impl Media {
    fn description (&self) -> String {
        /*if let Media::Book { title,author} = self 
        {
            format!("Book : {} | Author : {}",title,author)
        }
        else if let Media::Movie { title,director  } = self 
        {
            format!("Movie : {} | Director : {}",title,director)
        }
        else if let Media::Audiobook {title} = self 
        {
            format!("Audiobook : {}",title)
        }
        else {
                String::from("Unknown Media")
             }*/

        match self {
            Media::Book {title,author} =>
            {
                format!("The Book is '{}' and The Author is '{}'",title,author)
            }
            
            Media::Movie {title,director } => 
            {
                format!("The movie is '{}' and The Director is '{}'",title,director)
            }

            Media::Audiobook {title} => 
            {
                format!("The Audiobook is '{}'",title)
            }
            Media::Stringlerin_arasına_int_denemesi(episode_number) => {
                format!("{}",episode_number)
            }
            Media::Boş_metin => {
                format!("Boş line")
            }

        }
            
     }


  }
#[derive(Debug)]
  struct Catalog {
    items:Vec<Media>
  }
  impl Catalog {
    fn new () -> Self {
        Catalog { items: vec![]}
    }
    fn add_media (&mut self, media:Media) {
        self.items.push(media);
    }
      
  }
    


fn print_media(media: Media) {
    println!("{:#?}", media)
}
fn main() {
    let audiobook= Media::Audiobook { 
        title: String::from("An audiobook"),
        };
    

    let book = Media::Book {
         title:String::from("Crime and Punishment"), 
         author: String::from("Dostoevski") 
        };


    let  movie = Media::Movie { 
        title:String::from("The Green Mile"),
        director:String::from (" Frank Darabont") 
    };    
    let stringlerin_arasına_int =Media::Stringlerin_arasına_int_denemesi(15) ;
    let boş_metin=Media::Boş_metin;

    println!("{}",book.description());
    println!("{}",movie.description());
    println!("{}",audiobook.description());
    println!("{}",stringlerin_arasına_int.description());
    println!("{}",boş_metin.description());


    print_media(audiobook);
    print_media(book);
    print_media(movie); 


    /*let mut  catalog = Catalog::new();
    catalog.add_media(movie);
    catalog.add_media(book);
    catalog.add_media(audiobook);
    catalog.add_media(stringlerin_arasına_int);
    catalog.add_media(boş_metin);

    println!("{:#?}",catalog);*/
   
}
