const PATH: &str = ".";
const DATI: &str = "data";
const NOMI: &str = "nomi";
const RES: &str = "result";

use super::Studente;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::io::{Error, Write};

///Restituisce i file nella cartella superiore come OsString
fn get_files() -> Vec<String> {
    let mut pathbuf=PathBuf::from(PATH);
    pathbuf.push(DATI);

    
    if let Ok(dir) = fs::read_dir(pathbuf) {
        dir.filter_map(|entry| {
            if let Ok(entry) = entry {
                if let Ok(filetype) = entry.file_type() {
                    if filetype.is_file() {
                        return Some(entry.file_name().into_string().unwrap_or("N/A".to_owned()));
                    }
                }
            }
            return None;
        })
        .collect()
    } else {
        vec![]
    }
}

fn get_materia(string: &String) -> String {
    string.split("_").next().unwrap_or("N/A").to_owned()
}

fn read_file(path: &str) -> Result<Vec<Studente>, Error> {
    let mut pathbuf=PathBuf::from(PATH);
    pathbuf.push(DATI);
    pathbuf.push(path);
    pathbuf.set_extension("csv");
    Ok(
        fs::read_to_string(pathbuf)?
            .split("\n")
            .map(|s|{
                s.trim()
            })
            .skip(1)
            .filter_map(|line| match Studente::try_from((line, path)) {
                Ok(stud) => Some(stud),
                Err(_) => {
                    None
                },
            })
            .collect(),
    )
}

fn merge_students() -> HashMap<u32, Studente> {
    let materie = get_materie();

    let mut students: HashMap<u32, Studente> = HashMap::new();

    for materia in &materie {
        if let Ok(studenti) = read_file(materia) {
            for studente in studenti {
                let matricola = studente.matricola;
                match students.get_mut(&matricola) {
                    Some(stud) => {
                        stud.voti.extend(studente.voti.into_iter());
                    }
                    None => {
                        students.insert(matricola, studente);
                    }
                }
            }
        }
    }

    students
}

fn merge_names(students: &mut HashMap<u32, Studente>) {

    let mut pathbuf=PathBuf::from(PATH);
    pathbuf.push(NOMI);
    pathbuf.set_extension("csv");

    if let Ok(content) = fs::read_to_string(pathbuf) {
        content
            .split("\n")
            .map(|s|{
                s.trim()
            })
            .skip(1)
            .filter_map(|line| {
                if let Some((matricola, nome)) = line.clone().split_once(",") {
                    if let Ok(matricola) = matricola.parse() {
                        return Some((matricola, nome.to_owned()));
                    }
                }
                return None;
            })
            .for_each(|(matricola, nome)| {
                if let Some(student) = students.get_mut(&matricola) {
                    student.nome = Some(nome);
                } else {
                    let stud = Studente {
                        nome: Some(nome),
                        matricola: matricola,
                        ..Default::default()
                    };
                    students.insert(matricola, stud);
                }
            })
    }
}

pub fn get_students() -> Vec<Studente> {
    let mut students = merge_students();
    merge_names(&mut students);

    students.into_values().collect()
}

fn get_materie() -> Vec<String> {
    let materie: Vec<String> = get_files()
        .iter()
        .filter_map(|s| {
            if let Some(name) = s.strip_suffix(".csv") {
                Some(name.to_owned())
            } else {
                None
            }
        })
        .collect();
    materie
}

pub fn store_students(students: &Vec<Studente>) -> Result<(), Error> {
    let materie: Vec<String> = get_materie().iter().map(|s| get_materia(s)).collect();

    let mut header = format!("{:10}", "matricola");
    for materia in &materie {
        header += &format!(",{materia:5}");
    }
    header += ",nome\n";

    let mut pathbuf=PathBuf::from(PATH);
    pathbuf.push(RES);
    pathbuf.set_extension("csv");

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(pathbuf)?;

    file.write(header.as_bytes())?;
    for studente in students {
        let mut line = format!("{:10}", studente.matricola.to_string());
        for materia in &materie {
            let voto;
            if let Some(v) = studente.voti.get(materia) {
                voto = v.to_string();
            } else {
                voto = String::new();
            }
            line += &format!(",{voto:5}");
        }

        let nome = &studente.nome.clone().unwrap_or_default();
        line += &format!(",{nome}\n");

        file.write(&line.as_bytes())?;
    }

    Ok(())
}
