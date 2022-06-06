use std::{cmp::Ordering, collections::HashMap, hash::Hash};

pub mod io;

#[derive(Debug, Default)]
pub struct Studente {
    pub nome: Option<String>,
    pub matricola: u32,
    pub voti: HashMap<String, Voto>,
}

impl Studente {
    ///Restituisce un punteggio che dipende dai voti ottenuti
    ///e dai crediti di ogni esame
    fn get_score(&self) -> u64 {
        let mut acc = 0;
        for voto in self.voti.values() {
            if voto.voto != 0 && voto.voto != 1 {
                acc += voto.voto as u64 * voto.materia.cfu as u64;
            }
        }
        acc
    }
}

impl PartialOrd for Studente {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}
impl Ord for Studente {
    ///Sono uguali se hanno la stessa matricola,
    ///altrimenti li ordina in base al punteggio
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.matricola == other.matricola {
            return Ordering::Equal;
        }
        if self.get_score() >= other.get_score() {
            return Ordering::Greater;
        } else {
            return Ordering::Less;
        }
    }
}
impl PartialEq for Studente {
    fn eq(&self, other: &Self) -> bool {
        self.matricola == other.matricola
    }
}
impl Eq for Studente {}

impl Hash for Studente{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.matricola.hash(state);
    }
}

pub struct InvalidStudent;

impl TryFrom<(&str, &str)> for Studente {
    type Error = InvalidStudent;

    ///Crea uno studente a partire da un'input del tipo ("matricola,voto","materia_cfu").
    ///
    ///Es: `Student::try_from(("559298,27","FIS_12"))`.
    ///
    ///Restituisce un errore del tipo `InvalidStudent` se i dati non sono nel formato giusto
    fn try_from((value, path): (&str, &str)) -> Result<Self, Self::Error> {
        //Ricavo i dati necessari dai due parametri
        let (materia, cfu) = path.split_once("_").ok_or(InvalidStudent)?;
        let (matricola, voto) = value.split_once(",").ok_or(InvalidStudent)?;

        //Converto da stringa nei valori che mi servono
        let matricola = matricola.parse().or(Err(InvalidStudent))?;
        let voto = voto.parse().or(Err(InvalidStudent))?;

        return Ok(Studente {
            nome: None,
            matricola,
            voti: HashMap::from([(
                materia.to_owned(),
                Voto {
                    materia: Materia {
                        nome: materia.to_owned(),
                        cfu: cfu.parse().or(Err(InvalidStudent))?,
                    },
                    voto,
                },
            )]),
        });
    }
}

#[derive(Debug)]
pub struct Voto {
    materia: Materia,
    voto: u8,
}

impl ToString for Voto {
    fn to_string(&self) -> String {
        format!("{}", self.voto)
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Materia {
    nome: String,
    cfu: u8,
}
