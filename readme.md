
# Matricolone
È un progetto molto prematuro e fatto in un paio di giornate, è senza pretese e purtroppo non esistono abbastanza dati per goderselo.
L'eseguibile si trova nella cartella **`./`** e va a cercare i dati sugli esami nella cartella **`./data_real`** (nel formato *`matricola,voto`*, inoltre il nome dei file dev'essere nel formato **`./data_real/materia_cfu.csv`**). 
I dati sui nomi invece si trovano in **`./nomi.csv`**.
Il risultato è piazzato in **`./result.csv`**
Un esempio è:
```
folder
│   matricolone.exe
│   nomi.csv    
│	result.csv
└───data_real
    |   ASE_6.csv
    |   FDA_9.csv
	|	...
```
Dove **`nomi.csv`** è nel formato:
```
matricola,nome
559298,Alex Bonini
512377,Marco Rossi
559320,Giorgio Bianchi
566835,Lorenzo Lorenzi
561883,Gigi Buffon
...
```
E **`ASE_6.csv`** (Il 6 indica il numero di cfu del corso, che sarà usato nel calcolo della classifica in **`result.csv`**) è nel formato:
```
matricola,nome
559298,27
512377,25
559320,18
566835,29
561883,22
```
Il risultato sarà del tipo:
```
>matricola ,ase  ,ee   ,fdi  ,tlc  ,nome
560124     ,30   ,31   ,31   ,31   ,
560654     ,29   ,31   ,30   ,31   ,
559320     ,29   ,29   ,31   ,31   ,Mariangela Verdi
562335     ,28   ,28   ,31   ,31   ,
559549     ,26   ,29   ,29   ,31   ,
```

## Compilazione
Basta fare **`cargo run`** nella cartella del progetto, serve avere Rust installato.
