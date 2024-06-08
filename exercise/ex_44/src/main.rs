// File Processing with Enums and Patterns Let's consider a file processing scenario where you need to analyze different file types. We can use enums and pattern matching to handle each file type accordingly:

enum FileType {
    Text,
    Video,
    Image,
}


fn display_file(filteType:FileType , doc_name:&String) {
    match filteType {
        FileType::Text => println!("File name : {doc_name}, Type is Text"),
        FileType::Video => println!("File name : {doc_name}, Type is Video"),
        FileType::Image => println!("File name : {doc_name}, Type is Image"),
    }
}

fn main() {
    display_file(FileType::Text , &"doc.txt".to_string());
    display_file(FileType::Video , &"vid.mp4".to_string());
    display_file(FileType::Image , &"image.png".to_string());
}
