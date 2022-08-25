# BetaDev Game
Velkommen til readme filen til spillet, her skal vi fort gå over hva som skal gjøres, hva som er planen og hva spillet går ut på

Dette sammarbeidet er laget for at nye folk i BETA kan bli mere erfaren på å lage GitHub projekter og spill. Ikke minst sammarbeide 

# HVA SPILLET GÅR UT PÅ
Spillet er et spill som består av mange mini spill. 
Konseptet som er planlagt er å kunne gå rundt, velge forskjellige spill og få poeng ut i fra det.
Noe som er mulig er å låse noen spill til man får så og så mange poeng. 
Spillene er spill som pac-man, tetris, space invader også videre. 

# HVA SKAL GJØRES
Her handler det om å følge med på "issues" på github, der lager vi project og annet som skal gjøres.

# GAME LOOP
Spillet består av mini-games, også et hoved-lobby-spill, hvor vinneren kan velge neste spill.

#### Lobby-spillet
Tenkte at det skulle være som i Squid-Game, at man skal hoppe over en bro, også må man huske hvilke plater som er trygge å gå på.
* hvis du tråkker på en løs plate; detter du, dør, og må starte fra scratch.
* hvis du tråkker på TRE trygge plater, kan du velge neste mini-game
* i mini-games kan du få penger, som du kan bruke til å kjøpe kule ting
* når første mann er over brua, starter end game:
  * han kan skyte ned de andre
  * de skal prøve å løpe over uten å dø
  * de som klarer det, kan bruke butikken
  
 #### Endgame
 Arena fight, pickups som koster mye penger i starten.
 
 # stil / code standard
 kode står på engelsk (så slipper man å oversette "update", "init" o.l.)
 kommentarer står på norsk (lettere å lese og skrive)
 navn følger Rust code standard: 
 * variabler, funksjoner og moduler er 'snake_case',
 * typer er 'UpperCamelCase'
 
