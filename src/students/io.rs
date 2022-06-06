const PATH: &str = ".";
const DATI: &str = "data";
const NOMI: &str = "nomi";
const RES: &str = "result";

use super::Studente;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{Error, Write};
use std::path::PathBuf;

///Restituisce i file nella cartella ./data nel formato `ASE_6.csv`
fn get_files() -> Vec<String> {
    let mut pathbuf = PathBuf::from(PATH);
    pathbuf.push(DATI);

    if let Ok(dir) = fs::read_dir(pathbuf) {
        dir.filter_map(|entry| {
            //Prende solo i file e ignora le cartelle
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

///Passa da una cosa del tipo `ASE_6.csv/ASE_6` ad `ASE`
fn get_materia(string: &String) -> String {
    string.split("_").next().unwrap_or("N/A").to_owned()
}

///Legge un file e restituisce una lista di studenti basati sul formato `matricola,voto`
fn read_file(path: &str) -> Result<HashSet<Studente>, Error> {
    let mut pathbuf = PathBuf::from(PATH);
    pathbuf.push(DATI);
    pathbuf.push(path);
    pathbuf.set_extension("csv");


    let studenti=fs::read_to_string(pathbuf)?
        .split("\n")
        .map(|s| {
            s.trim() //Il trim serve a sbarazzarsi di eventuali \r
        })
        .skip(1) //skippo l'header
        .filter_map(|line| match Studente::try_from((line, path)) {
            Ok(stud) => Some(stud),
            Err(_) => None,
        })
        .collect();

    Ok(studenti)
}

///Prende gli studenti da ognuno dei csv e unisce i voti di quelli che hanno la stessa matricola
fn merge_students() -> HashMap<u32, Studente> {
    let materie = get_materie();

    //uso una mappa per essere più efficiente :))
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

///Legge i nomi noti e li associa alle matricole già presenti
fn merge_names(students: &mut HashMap<u32, Studente>) {
    let mut pathbuf = PathBuf::from(PATH);
    pathbuf.push(NOMI);
    pathbuf.set_extension("csv");

    if let Ok(content) = fs::read_to_string(pathbuf) {
        content
            .split("\n")
            .map(|s| s.trim()) //per togliere l'eventuale \r
            .skip(1) //skip header
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
                        matricola,
                        ..Default::default()
                    };
                    //Se lo studente non è presente lo aggiungo comunque
                    students.insert(matricola, stud);
                }
            })
    }
}

///Ottiene una lista completa degli studenti con nomi e voti
pub fn get_students() -> Vec<Studente> {
    let mut students = merge_students();
    merge_names(&mut students);

    students.into_values().collect()
}

///Passa da cose del tipo `ASE_6.csv` ad `ASE_6`
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

///Salva gli studenti nel file
pub fn store_students(students: &Vec<Studente>) -> Result<(), Error> {
    let materie: Vec<String> = get_materie().iter().map(|s| get_materia(s)).collect();

    let mut header = format!("{:10}", "matricola");
    for materia in &materie {
        header += &format!(",{materia:5}");
    }
    header += ",nome\n";

    let mut pathbuf = PathBuf::from(PATH);
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
