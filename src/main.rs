extern crate xml;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;

use xml::reader::{EventReader, XmlEvent};

#[derive(Debug, Clone)]
pub struct Cheat {
    pub name: String,
    pub code: String,
}
#[derive(Debug, Clone)]
pub struct Category {
    pub name: String,
    pub id: String,
    pub cheats: Vec<Cheat>,
}

#[derive(Debug, PartialEq)]
pub enum Tags {
    Codelist,
    GameId,
    Folder,
    FolderName,
    FolderAllowedon,
    Cheat,
    CheatName,
    CheatNote,
    CheatCodes,
    Other,
}

#[derive(Debug, PartialEq)]
pub enum CheatTags {
    Cheat,
    CheatName,
    CheatNote,
    CheatCodes,
    Other,
}
fn main() {
    let game_id = env::args().nth(1).unwrap();
    let cheat_xml_path = env::args().nth(2).unwrap();
    let output_path = env::args().nth(3).unwrap();
    
    let file = File::open(cheat_xml_path).unwrap();
    let file = BufReader::new(file);
    let mut categories: Vec<Category> = vec![];

    let parser = EventReader::new(file);

    let mut new_cheat = Cheat {
        name: String::from("(new AR code)"),
        code: String::new(),
    };
    let mut new_category = Category {
        name: String::from("(new category)"),
        id: String::new(),
        cheats: vec![],
    };

    let mut tag = Tags::Other;
    let mut cheat_tags = CheatTags::Other;
    let mut is_cheat = false;
    let mut w = File::create(output_path).unwrap();
    let mut is_the_game = false;
    let mut skip_the_rest = false;
    for e in parser {
        if skip_the_rest {
            break;
        }
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => match name.local_name.as_str() {
                "gameid" => {
                    if is_the_game {
                        skip_the_rest = true;
                        break;
                    }
                    tag = Tags::GameId;
                    is_the_game = false;
                }
                "folder" => {
                    tag = Tags::Folder;
                    is_cheat = false;
                }
                n if n == "name" && tag == Tags::Folder => {
                    tag = Tags::FolderName;
                    is_cheat = false;
                }
                n if n == "allowedon" && (tag == Tags::Folder || tag == Tags::FolderName) => {
                    tag = Tags::FolderAllowedon;
                    is_cheat = false;
                }
                n if n == "cheat" && (tag == Tags::FolderName || tag == Tags::FolderAllowedon) => {
                    cheat_tags = CheatTags::Cheat;
                    is_cheat = true;
                }
                n if n == "name" && cheat_tags == CheatTags::Cheat => {
                    cheat_tags = CheatTags::CheatName;
                    is_cheat = true;
                }
                n if n == "note" && cheat_tags == CheatTags::Cheat => {
                    cheat_tags = CheatTags::CheatNote;
                    is_cheat = true;
                }
                n if n == "codes" && cheat_tags == CheatTags::Cheat => {
                    cheat_tags = CheatTags::CheatCodes;
                    is_cheat = true;
                }
                _ => {}
            },
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            Ok(XmlEvent::Characters(name)) => {
                if tag == Tags::GameId && name.contains(game_id.as_str()) {
                    is_the_game = true;
                }

                if is_the_game {
                    if is_cheat {
                        match cheat_tags {
                            CheatTags::CheatName => {
                                new_cheat.name = name;
                                cheat_tags = CheatTags::Cheat;
                            }
                            CheatTags::CheatNote => {
                                new_cheat.name = new_cheat.name + " " + name.as_str();
                                cheat_tags = CheatTags::Cheat;
                            }
                            CheatTags::CheatCodes => {
                                new_cheat.code = name;
                                cheat_tags = CheatTags::Cheat;
                            }
                            _ => (),
                        }
                        if new_cheat.name != "" && new_cheat.code != "" {
                            new_category.cheats.push(new_cheat.clone());
                            new_cheat = Cheat {
                                name: String::from("(new AR code)"),
                                code: String::new(),
                            };
                        }
                    } else {
                        match tag {
                            Tags::FolderName => {
                                if new_category.cheats.len() > 0 {
                                    categories.push(new_category.clone());
                                }
                                new_category.name = name;
                                new_category.cheats = vec![];
                            }
                            _ => (),
                        }
                    }
                }
            }
            _ => {}
        }
    }

    categories.push(new_category);

    for category in categories {
        writeln!(&mut w, "CAT {}", category.name).unwrap();
        writeln!(&mut w, "").unwrap();

        for cheat in category.cheats {
            writeln!(&mut w, "CODE 0 {}", cheat.name).unwrap();
            let codes: Vec<&str> = cheat.code.split(" ").collect();
            codes.chunks(2).for_each(|code| {
                writeln!(&mut w, "{} {}", code[0], code[1]).unwrap();
            });
            writeln!(&mut w, "").unwrap();
        }
    }
}
