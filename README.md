# Predlog projekta (V2)

## Pametni asistent za društvenu igru Wingspan - Big Bird Brain

### Dušan Komadinović SV65/2022

# Opis problema

## Motivacija

Društvena igra _Wingspan_ predstavlja takmičarsku stratešku _engine-building_ igru za dva do pet igrača. U njoj igrači razvijaju sopstveni ekosistem ptica kroz pažljivo planiranje poteza i upravljanje resursima sa ciljem da prikupe što veći broj bodova. Partija traje četiri runde sa ukupno 26 poteza, gde se broj poteza smanjuje iz runde u rundu.

Efikasnost odluke ne zavisi samo od trenutne koristi. Strateška dubina igre proizilazi iz velikog broja međusobno zavisnih faktora koji utiču na vrednost svakog poteza, kao što su:

- već odigrane ptice i njihove sposobnosti,
- dostupni resursa hrane,
- sinergije (_synergy_) između staništa,
- ciljevi na kraju runde,
- preostali broj poteza u partiji i
- indirektni uticaj poteza protivnika.

Kompleksnost igre počinje i **pre prvog poteza**: na početku partije svaki igrač dobija pet nasumičnih karata ptica i dve karte bonus ciljeva. Moraju da odaberu do 5 ptica + različitih resursa (npr. tri ptice, crva i ribu) i jedan od dva cilja. Ova odluka može da ima dugoročni uticaj na celu strategiju, jer direktno određuje šta igrač može da razvija od staništa i bonus cilj ka kojem teži celu partiju.

Zbog ovakve strukture odlučivanja, početnici i igrači srednjeg nivoa imaju poteškoća da procene koja akcija donosi najveću dugoročnu vrednost. Moraju da analiziraju veliki broj varijabli i posledica budućih poteza, što nije lako ni za iskusnije igrače.

Ovakav domen predstavlja pogodan problem za primenu sistema baziranih na znanju, jer odluke koje donose iskusni igrači počivaju na skupu heurističkih pravila i ekspertskog iskustva, kao što su:

- rano razvijanje efikasnog _engine_-a,
- optimizacija resursa,
- prilagođavanje strategije ciljevima runde,
- balansiranje kratkoročne i dugoročne koristi.

Motivacija ovog projekta je razvoj **pametnog asistenta zasnovanog na znanju** koji formalizuje strateško razmišljanje igrača i omogućava njegovu primenu pri analizi konkretnih situacija tokom partije. Cilj je kreiranje sistema koji može da pomogne igračima u razumevanju posledica svojih odluka i prepoznavanju kvalitetnih strateških izbora poteza koji možda nisu odmah očigledni. Ujedno može da posluži kao alat za učenje strategije i analize partija.

### Napomena

Postoje razne ekspanzije za igru koje uvode nova pravila i mehanike igre. Fokus projekta je samo na osnovnoj igri.

# Pregled problema

Tokom jednog poteza u _Wingspan_-u igrač bira između četiri osnovne akcije:

- igranje nove ptice u jedno od staništa,
- uzimanje hrane,
- polaganje jaja,
- uzimanje novih karata ptica.

Iako je skup mogućih akcija ograničen, procena njihove stvarne vrednosti predstavlja složen problem odlučivanja. Optimalan potez zavisi od velikog broja međusobno povezanih faktora, uključujući:

- trenutno razvijen _engine_,
- dostupne resurse,
- ciljeve runde i
- preostali broj poteza.

Vrednost pojedinačne akcije često se ne može proceniti izolovano, već isključivo u kontekstu kompletnog stanja igre.

Problem koji se rešava ovim projektom može se formulisati na sledeći način:

> Kako formalizovati ekspertsko strateško znanje igrača _Wingspan_-a i iskoristiti ga za generisanje obrazložene preporuke optimalne akcije u datom stanju igre?

## Postojeća rešenja

Postojeći digitalni alati vezani za _Wingspan_ mogu se svrstati u dve osnovne kategorije:

1. **Referentni alati:**
   - [baze podataka](https://navarog.github.io/wingsearch/) o kartama,
   - sajtovi za [strategije i preporuke](https://wingsplain.com/),
   - digitalni [priručnici pravila](https://wingspan.rulepop.com/) i
   - sajtovi za [vođenje rezultata](https://tablegameshub.com/wingspan-score-calculator/).

   Ovi alati služe kao pomoć pri učenju pravila ili evidenciji partije, ali ne pružaju stratešku analizu niti preporuke za donošenje odluka.

2. **Digitalna adaptacija igre**

   Zvanična [digitalna verzija](https://store.steampowered.com/app/1054490/Wingspan/) igre omogućava igranje protiv računara ili drugih igrača, ali ne obrazložava razloge iza poteza niti modeluje proces strateškog razmišljanja.

Iako ovi alati olakšavaju igranje, nijedan od njih ne funkcioniše kao **inteligentni savetodavni sistem** koji analizira stanje partije i pomaže igraču da razume zašto je određeni potez dobar ili loš. **Ne postoji javno dostupno rešenje** zasnovano na pravilima koje prihvata strukturisano stanje igre i generiše rangiranu, obrazloženu listu preporučenih poteza.

## Prednost predloženog rešenja

Predloženi sistem bi sadržao:

1. **Sveobuhvatno rezonovanje**

   Sistem analizira celokupno stanje igre umesto izolovanih elemenata poput pojedinačnih karata ili trenutnog rezultata.

2. **Rangirane i obrazložene preporuke**

   Sistem generiše listu mogućih akcija zajedno sa obrazloženjem procesa zaključivanja i aktiviranih pravila.

3. **Transparentnu bazu znanja**

   Znanje je predstavljeno kroz eksplicitna pravila koja se mogu proveravati, menjati i proširivati, što sistem čini pogodnim za učenje i analizu strategije.

# Metodologija rada

Predloženi sistem predstavlja **ekspertski savetodavni sistem** koji nad formalizovanim opisom trenutnog stanja igre generiše preporuke poteza korišćenjem baze znanja i mehanizama rezonovanja.

## Ulazi u sistem (_Input_)

Ulaz u sistem predstavlja strukturisani opis trenutnog stanja partije. Sistem razlikuje dva moda unosa koji odgovaraju fazama igre.

### _Draft Phase_ - početno stanje

Da bi igrač inicijalizovao stanje igre i dobio preporuku koje karte i resurse da zadrži na samom početku, mora da unese:

- **Inicijalnu ruku:** pet izvučenih karata ptica,
- **Bonus karte (skriveni ciljevi):** dve izvučene bonus karte (bira jednu od njih),
- **Stanje hranilice (_Bird-feeder_):** trenutno dostupni simboli na pet kocki hrane,
- **Ptice u _Bird-tray_-u:** tri uvek vidljive karte ptice (svaka uzeta se menja sa novom na kraju igračevog poteza, a na prelazu rundi se menjaju sve tri),
- **Ciljevi za sve runde (_end-of-round goals_) i tip bodovanja (_casual_ ili _competitive_)**,
- **Broj igrača u partiji**.

### _Main Phase_ - ostatak partije

#### Stanje partije

- broj preostalih poteza u rundi,
- trenutno stanje hrane u _bird-feeder_-u,
- trenutne ptice u _bird-tray_-u i
- nivo ispunjenja aktivnog _end-of-round_ cilja od strane protivnika.

#### Tabla igrača

- ptice postavljene po staništima (šuma, livada, močvara),
- **popunjenost kolone** svakog staništa (direktno određuje vrednost akcije staništa i cenu igranja novih karata),
- broj položenih jaja po pticama,
- keširana hrana na pticama i
- _tuck_-ovane karte.

#### Resursi igrača

- dostupni tokeni hrane po tipu (crv, bobice, riba, miš, pšenica),
- trenutne karte ptica u ruci.

#### Strateški kontekst

- bonus karte igrača, sa trenutnim procentualnim i numeričkim napretkom ka ispunjenju ciljeva,
- vidljivo stanje protivnika (broj ptica po staništima i njihov napredak u aktuelnom cilju runde).

## Izlazi iz sistema (_Output_)

Na osnovu ulaznih činjenica sistem generiše **preporuku poteza**.

### Preporuka u fazi odabira karata

Rangirane preporuke koje sadrže:

- **Selekcija karata:** tačan skup ptica koje treba zadržati (od 0 do 5) i bonus kartu,
- **Selekcija resursa:** tačan skup početnih tokena hrane koje treba zadržati (po pravilu igre: broj zadržanih ptica + broj zadržanih tokena hrane mora biti tačno 5; tokeni svi različiti),
- **Skor:** numerička evaluacija sinergije između ruku, odabranog bonusa i spoljašnjih faktora (stanja table),
- **Obrazloženje rezonovanja**

Primer izlaza jedne preporuke:

> **Preporuka:** Zadržati bonus kartu _Bird Feeder_, ptice _A_, _B_, _C_ i resurse _crv_ i _pšenicu_
>
> **Skor:** 14.5
>
> **Obrazloženje:** Bonus karta obuhvata velik broj ptica (44% karata). Ptica _A_ zahteva pšenicu. Ptica _B_ zahteva crva i pšenicu. Ptice _A_ i _B_ doprinose cilju 1. runde (najviše ptica u šumi). Ptica _C_ je iznad proseka i treba je sačuvati. Nedostajući resursi dostupni u _birdfeeder_-u.

### Preporuka u glavnoj fazi igre

Rangirane preporuke za akcije koje sadrže:

- **Konkretnu akciju**,
- **Skor:** numerička evaluacija preporučene akcije,
- **Obrazloženje rezonovanja**,

Primer izlaza jedne akcije:

> **Preporuka:** Igrati pticu _X_ u staništu livade
>
> **Skor:** 8.7
>
> **Obrazloženje:** Poboljšava produkciju jaja. Izjednačava igrača sa liderom u cilju runde. Doprinosi ispunjenju bonus cilja _W_.

## Baza znanja

Baza znanja predstavlja centralni deo sistema i sastoji se iz statičkog i dinamičkog znanja.

### Činjenice (_Facts_)

#### Statičko znanje

1. Tabela vrednosti akcije po popunjenosti reda:

   Vrednost akcije svakog staništa i cena igranja ptica direktno zavisi od broja ptica u tom redu. Ova tabela je ključni deo statičkog znanja jer omogućava sistemu da proceni **stvarnu vrednost akcije**.

   Svi braon efekti ptica se aktiviraju kada se uradi jedna od tri akcije uzimanja resursa u tom redu. Posle prve postavljene karte, na neparnom broju ptica, mogu da se zamene drugi resursi da bi se uzeo još jedan iz tog reda (obeleženi sa +1).

   | Popunjenost reda | Cena ptice (jaja) | Vrednost akcije u Šumi (hrana) | Vrednost akcije u Livadi (jaja) | Vrednost akcije u Močvari (karte) |
   | ---------------- | ----------------- | ------------------------------ | ------------------------------- | --------------------------------- |
   | 0 ptica          | /                 | 1                              | 2                               | 1                                 |
   | 1 ptica          | 0                 | 1+1                            | 2+1                             | 1+1                               |
   | 2 ptice          | 1                 | 2                              | 3                               | 2                                 |
   | 3 ptice          | 1                 | 2+1                            | 3+1                             | 2+1                               |
   | 4 ptice          | 2                 | 3                              | 4                               | 3                                 |
   | 5 ptica          | 2                 | 3+1                            | 4+1                             | 3+1                               |

2. Broj poteza po rundama:
   - 8 poteza u prvoj rundi,
   - 7 poteza u drugoj rundi,
   - 6 poteza u trećoj rundi i
   - 5 poteza u četvrtoj rundi.

   Sistem mora da zna koliko još poteza može da odigra u rundi da bi izračunao da li neka akcija iz više koraka koja doprinosi trenutnoj rudni može da se izvede (npr. da odigra pticu za koju nema dovoljno hrane i jaja, trebaju mu dva poteza da skupi resurse i treći da je odigra). Strategija se menja kroz runde.

3. Baza podataka karata ptica (170 ptica osnovne igre), modelovana minimalno sledećim atributima:
   - naziv,
   - cena hrane:
     - bez cene,
     - jedan resurs,
     - jedan od dva ponuđena resursa,
     - dva resursa (ista ili različita),
     - tri resursa (ista ili različita)
   - stanište u koje može da se igra (jedno ili više),
   - raspon krila,
   - broj bodova,
   - tip gnezda:
     - bez gnezda,
     - _platform_,
     - _cup_,
     - _cavity_,
     - _ground_,
     - zvezda (bilo koji tip, može da se prilagođava aktuelnom cilju i cilju bonus karata)
   - kapacitet gnezda (ako ima gnezdo; broj jaja koji može da čuva),
   - **tip sposobnosti**:
     - bez sposobnosti,
     - **bela** - pri igranju (_when played_),
     - **braon** - aktivacija staništa (_when activated_) i
     - **roza** - između poteza protivnika (_once between turns_, pasivni prihod)

     Kategorije mogućih efekata sposobnosti:
     - generisanje hrane,
     - polaganje jaja,
     - zamena resursa:
       - hrana za drugu hranu,
       - hrana za kartu,
       - jaje za hranu,
       - jaje za kartu(e),
       - karta za kartu
     - vučenje karata ptica ili bonus karata,
     - keš mehanika,
     - _tuck_ mehanika
     - igranje dodatne karte ptice,
     - premeštanje ptice u drugo stanište,
     - ponavljanje sposobnosti i efekata,

4. Baza podataka karata bonus ciljeva

   Bonus karte predstavljaju skrivene dugoročne ciljeve, svaki igrač počinje sa jednim bonus ciljem koji samo on zna, a u toku partije može izvući nove.

   Svaka bonus karta treba da bude modelovana kao strukturisani skup uslova:
   - naziv karte,
   - uslov bodovanja,
   - atribut koji se proverava,
   - način računanja bodova.

   Primeri:
   - broj ptica određene kategorije,
   - ptice sa određenim tipom gnezda,
   - ptice određenog raspona krila,
   - ptice u određenom staništu.

5. Baza _end-of-round_ ciljeva

   U svakoj rundi igrači se nadmeću da što bolje ispune cilj za tu rundu. Mogu se skupljati u _casual_ (plava strana, _points-per-item_) i _competitive_ režimu (zelena strana, _majority_), pri čemu je drugi oštriji pri bodovanju. Odabran režim određuje način računanja bodova po rundama i kriterijum poređenja između igrača.

   Varijante ciljeva:
   - ukupan broj ptica u nekom od staništa (šumi, livadi, močvari),
   - ukupan broj jaja u nekom od staništa (šumi, livadi, močvari),
   - broj jaja u gnezdu određenog tipa (_platform_, _cup_, _cavity_, _ground_),
   - ptice sa gnezdom određenog tipa (_platform_, _cup_, _cavity_, _ground_) koje imaju bar jedno jaje na sebi
   - ukupan broj ptica,
   - setovi jaja po staništima (ukupan broj setova jaja po kolonama u svakom staništu, gde je set 1 jaje u svakom staništu)

   U _casual_ režimu u svakoj rundi igrači mogu dobiti od 0 do 5 bodova u zavisnosti od ispunjenosti cilja i svi mogu dobiti isti broj bodova.

   U _competitive_ igrači nadmeću da maksimalno ispune cilj po sledećem bodovanju:

   | Runda | Prvo mesto | Drugo mesto | Treće mesto | Ostala mesta |
   | ----- | ---------- | ----------- | ----------- | ------------ |
   | 1     | 4          | 1           | 0           | 0            |
   | 2     | 5          | 2           | 1           | 0            |
   | 3     | 6          | 3           | 2           | 0            |
   | 4     | 7          | 4           | 3           | 0            |

   Dodatno ograničenje za _competitive_: ako igrači dele mesto, broj bodava se smanjuje po formuli:
   - $\frac{\text{ostvareno mesto}+\text{sva preskočena}}{\text{broj igrača}}$, zaokruženo na dole
   - Primer: ako u igri sa tri igrača dva dele prvo mesto u prvoj rundi, dobiće samo po 1 bod - `(4+1)/3 = 1.67 -> floor(1.67) = 1`, a treći igrač ne dobija bodove
   - Primer: ako u igri sa četiri igrača tri dele prvo mesto u drugoj rundi, dobiće samo po 2 boda - `(5+2+1)/4 = 2`, a četvrti igrač ne dobija bodove

   Primer za isti cilj u različitom režimu igranja (broj jaja u šumi):
   - u _casual_ je dovoljno samo 5 jaja da se ostvari maksimum bodova, dok višak jaja neće doneti još bodova
   - u _competitive_ nema ograničenja za ispunjenost, pa će se igrači nadmetati da postave što više jaja u šumu

   Ove informacije omogućavaju sistemu da rezonuje o strategijskoj hitnosti i prilagođava preporuke u zavisnosti od aktivnog cilja runde.

#### Dinamičko znanje

Činjenice generisane iz korisničkog unosa:

- trenutno stanje table,
- raspoloživi resursi,
- stanje runde,
- napredak ka ciljevima i
- istorija poteza tokom partije.

## Pravila Faze Draftovanja

Ova pravila se aktiviraju isključivo u _Draft Phase_ i evaluiraju inicijalnu ruku igrača. Svaka kombinacija bonus karata i skupa ptica dobija numerički skor. Sastoji se od tri nivoa _forward chain_ pravila i _accumulate_ funkcije.

### Nivo 1 - Osnovno izvođenje činjenica

```
rule "DetectActivationPowerBird" no-loop {
    when
        DraftCombination.has_brown_or_pink_power_bird == true
    then
        add_fact("ACTIVATION_POWER_BIRD");
        log("Bird with brown or pink activation power detected in opening");
}

rule "DetectCardDrawEngine" no-loop {
    when
        DraftCombination.has_wetlands_card_draw_bird == true
    then
        add_fact("CARD_ENGINE");
        log("Wetlands card draw engine detected");
}

rule "DetectCheapBird" no-loop {
    when
        DraftCombination.cheap_bird_count > 0
    then
        add_fact("CHEAP_BIRD");
        log("Opening contains cheap early-game bird");
}

rule "DetectDeadOpening" no-loop {
    when
        DraftCombination.playable_bird_count == 0
    then
        add_fact("DEAD_OPENING");
        log("No playable birds detected");
}

rule "DetectExpensiveOpening" no-loop {
    when
        DraftCombination.expensive_bird_count >= 2
    then
        add_fact("EXPENSIVE_OPENING");
        log("Expensive opening detected (2+ birds with high cost)");
}

rule "DetectBonusSynergy" no-loop {
    when
        DraftCombination.bonus_match_count >= 2
    then
        add_fact("BONUS_SUPPORTED");
        log("Bonus synergy detected (2+ birds match bonus)");
}

rule "DetectFoodEngine" no-loop {
    when
        DraftCombination.has_food_generation_bird == true
    then
        add_fact("FOOD_ENGINE");
        log("Food engine detected");
}

rule "DetectFoodSupport" no-loop {
    when
        DraftCombination.required_food_available == true
    then
        add_fact("FOOD_SUPPORTED");
        log("Birdfeeder supports opening birds");
}

rule "DetectHighTierBird" no-loop {
    when
        DraftCombination.has_a_tier_bird == true
    then
        add_fact("HIGH_TIER_BIRD");
        log("High-tier (A-tier) bird detected");
}

rule "DetectRavenEngine" no-loop {
    when
        DraftCombination.has_egg_to_food_bird == true
    then
        add_fact("RAVEN_ENGINE");
        log("Raven-style egg conversion engine detected");
}

rule "DetectTopTierBird" no-loop {
    when
        DraftCombination.has_s_tier_bird == true
    then
        add_fact("TOP_TIER_BIRD");
        log("Top-tier bird detected");
}
```

##### Pravila sa _accumulate_*:

```
rule "CountPlayableBirds" {
    when
        DraftCombination.PlayableBirdsList != null
    then
        let count = accumulate(
            Bird.isPlayable == true
        );
        DraftCombination.setPlayableBirdCount(count);
        log("Counted playable birds");
}

rule "CountCheapBirds" {
    when
        DraftCombination.KeptBirds != null
    then
        let count = accumulate(
            Bird.foodCost <= 1
        );
        DraftCombination.setCheapBirdCount(count);
        log("Counted cheap birds");
}

rule "CountExpensiveBirds" {
    when
        DraftCombination.KeptBirds != null
    then
        let count = accumulate(
            Bird.foodCost >= 3
        );
        DraftCombination.setExpensiveBirdCount(count);
        log("Counted expensive birds");
}

rule "CountBonusMatches" {
    when
        DraftCombination.BonusCard != null
    then
        let count = accumulate(
            Bird.matchesBonus == true
        );
        DraftCombination.setBonusMatchCount(count);
        log("Counted bonus matches");
}

rule "DetectFoodSupportAvailability" {
    when
        DraftCombination.KeptBirds != null &&
        DraftCombination.Birdfeeder != null
    then
        let supported = accumulate(
            Bird.requiredFood subsetOf Birdfeeder.availableFood
        ) > 0;
        DraftCombination.setFoodSupported(supported);
        log("Evaluated food support availability");
}

rule "DetectTurn1PlayableBird" {
    when
        DraftCombination.KeptBirds != null &&
        DraftCombination.KeptFood != null
    then
        let playable = accumulate(
            Bird.canBePlayedWith(DraftCombination.KeptFood) == true
        ) > 0;
        DraftCombination.setHasTurn1PlayableBird(playable);
        log("Evaluated turn 1 playability");
}
```

### Nivo 2 - Strateško izvođenje činjenica

```
rule "DeriveHighTempoOpening" no-loop {
    when
        DraftCombination.has_turn1_playable_bird == true &&
        has_fact("CHEAP_BIRD") == true
    then
        add_derived_fact("HIGH_TEMPO");
        log("High-tempo opening derived");
}

rule "DeriveWetlandsCycling" no-loop {
    when
        has_fact("CARD_ENGINE") == true &&
        has_fact("FOOD_SUPPORTED") == true
    then
        add_derived_fact("WETLANDS_CYCLING");
        log("Wetlands cycling engine derived");
}

rule "DeriveRavenCombo" no-loop {
    when
        has_fact("RAVEN_ENGINE") == true &&
        DraftCombination.has_turn1_playable_bird == true
    then
        add_derived_fact("RAVEN_COMBO");
        log("Raven combo opening derived");
}

rule "DerivePremiumOpening" no-loop {
    when
        has_fact("TOP_TIER_BIRD") == true &&
        DraftCombination.has_turn1_playable_bird == true
    then
        add_derived_fact("PREMIUM_OPENING");
        log("Premium opening derived");
}

rule "DeriveGoodTierOpening" no-loop {
    when
        has_fact("HIGH_TIER_BIRD") == true &&
        DraftCombination.has_turn1_playable_bird == true
    then
        add_derived_fact("GOOD_TIER_OPENING");
        log("Good-tier opening derived (A-tier bird playable on turn 1)");
}

rule "DeriveEngineOpening" no-loop {
    when
        has_fact("ACTIVATION_POWER_BIRD") == true &&
        has_fact("FOOD_SUPPORTED") == true
    then
        add_derived_fact("ENGINE_OPENING");
        log("Engine-capable opening derived (activation power bird with food support)");
}
```

### Nivo 3 - Završna strateška evaluacija

```
rule "EvaluateHighTempoOpening" no-loop {
    when
        has_derived_fact("HIGH_TEMPO") == true
    then
        add_score(10.0);
        add_tag("HIGH_TEMPO");
        add_reason("Fast early-game development");
        log("High-tempo opening evaluated");
}

rule "EvaluateWetlandsCycling" no-loop {
    when
        has_derived_fact("WETLANDS_CYCLING") == true
    then
        add_score(14.0);
        add_tag("CARD_ENGINE");
        add_reason("Strong wetlands card engine");
        log("Wetlands cycling opening evaluated");
}

rule "EvaluateRavenCombo" no-loop {
    when
        has_derived_fact("RAVEN_COMBO") == true
    then
        add_score(16.0);
        add_tag("RAVEN_ENGINE");
        add_reason("Egg-to-food conversion engine enables flexible resource cycling");
        log("Raven combo evaluated");
}

rule "EvaluatePremiumOpening" no-loop {
    when
        has_derived_fact("PREMIUM_OPENING") == true
    then
        add_score(12.0);
        add_tag("PREMIUM_OPENING");
        add_reason("Contains top-tier playable bird");
        log("Premium opening evaluated");
}

rule "EvaluateGoodTierOpening" no-loop {
    when
        has_derived_fact("GOOD_TIER_OPENING") == true
    then
        add_score(8.0);
        add_tag("GOOD_TIER_OPENING");
        add_reason("Contains a playable A-tier bird with strong board presence");
        log("Good-tier opening evaluated");
}

rule "EvaluateEngineOpening" no-loop {
    when
        has_derived_fact("ENGINE_OPENING") == true
    then
        add_score(9.0);
        add_tag("ENGINE_OPENING");
        add_reason("Opening contains a brown/pink power bird supported by birdfeeder food");
        log("Engine opening evaluated");
}

rule "EvaluateBonusSynergy" no-loop {
    when
        has_fact("BONUS_SUPPORTED") == true
    then
        add_score(5.0);
        add_tag("BONUS_SYNERGY");
        add_reason("2+ opening birds match bonus card criteria - solid long-term scoring alignment");
        log("Bonus synergy evaluated");
}

rule "EvaluateSlowOpening" no-loop {
    when
        has_fact("EXPENSIVE_OPENING") == true
    then
        add_score(-8.0);
        add_tag("SLOW_OPENING");
        add_reason("Opening is too resource intensive");
        log("Slow opening evaluated");
}

rule "EvaluateDeadOpening" no-loop {
    when
        has_fact("DEAD_OPENING") == true
    then
        add_score(-15.0);
        add_tag("DEAD_OPENING");
        add_reason("No early playable birds");
        log("Dead opening evaluated");
}
```

## Pravila Glavne Faze

Ova pravila se aktiviraju isključivo u _Main Phase_.

### _Forward Chaining_ rezonovanje

Proces rezonovanja odvija se kroz četiri nivoa ulančavanja:

1. **Nivo 1** - detekcija strateške faze igre (EARLY / MID / LATE),
2. **Nivo 2** - validnost akcija (koje su akcije uopšte izvodljive),
3. **Nivo 3** - strateški zaključci (apstraktni zaključci iz stanja igre) i
4. **Nivo 4** - akumulacija prioriteta i generisanje rangirane preporuke.

_Accumulate_ operator koristi se za agregaciju znanja:

- sabiranje ukupne vrednosti staništa,
- procenu produkcione moći _engine_-a,
- izračunavanje strategijskog skora akcija i
- procenu napretka ka ciljevima.

Na osnovu akumuliranih vrednosti generišu se rangirane preporuke.

#### Nivo 1 - Detekcija strateške faze

Ova pravila se aktiviraju pre svih ostalih i klasifikuju trenutnu situaciju u jednu od tri strateške faze. Zaključak se koristi u svim narednim nivoima.

```
rule "DetectEarlyPhase" {
    when
        GameState.CurrentRound == 1 ||
        (GameState.CurrentRound == 2 && PlayerState.TotalBirdsPlayed <= 3)
    then
        GameState.setStrategicPhase("EARLY");
        log("Strategic phase determined as EARLY. Focusing on engine building.");
}

rule "DetectMidPhase" {
    when
        (GameState.CurrentRound == 2 && PlayerState.TotalBirdsPlayed > 3) ||
        GameState.CurrentRound == 3
    then
        GameState.setStrategicPhase("MID");
        log("Strategic phase determined as MID. Focusing on engine optimization and resource generation.");
}

rule "DetectLatePhase" {
    when
        GameState.CurrentRound == 4
    then
        GameState.setStrategicPhase("LATE");
        log("Strategic phase determined as LATE. Shifting focus entirely to raw victory points and egg spamming.");
}
```

#### Nivo 2 - Validnost akcija

Pravila ovog nivoa utvrđuju koje su akcije uopšte izvodljive u datom stanju igre.

```
rule "ValidatePlayBird" {
    when
        ActionEvaluation.ActionType == "PLAY_BIRD" &&
        PlayerState.HandCards.contains(ActionEvaluation.TargetBird) &&
        PlayerState.FoodTokens.matchesCost(ActionEvaluation.TargetBird.FoodCost) &&
        PlayerState.AvailableEggs >= ActionEvaluation.TargetHabitat.RequiredEggsForNextSlot
    then
        ActionEvaluation.setValid(true);
        log("Action PLAY_BIRD is valid for bird: " + ActionEvaluation.TargetBird.Name);
}

rule "ValidateGainFood" {
    when
        ActionEvaluation.ActionType == "GAIN_FOOD" &&
        GameState.BirdFeeder.HasDice == true
    then
        ActionEvaluation.setValid(true);
        log("Action GAIN_FOOD is valid.");
}

rule "ValidateLayEggs" {
    when
        ActionEvaluation.ActionType == "LAY_EGGS" &&
        PlayerState.TotalRemainingEggCapacity > 0
    then
        ActionEvaluation.setValid(true);
        log("Action LAY_EGGS is valid.");
}

rule "ValidateDrawCards" {
    when
        ActionEvaluation.ActionType == "DRAW_CARDS" &&
        GameState.CardTray.HasCards == true
    then
        ActionEvaluation.setValid(true);
        log("Action DRAW_CARDS is valid.");
}
```

#### Nivo 3 - Strateški zaključci

Pravila ovog nivoa izvode apstraktne strateške činjenice iz validacionih zaključaka i stanja igre.

```
rule "DeriveStrongForestEngine" {
    when
        GameState.StrategicPhase == "MID" &&
        PlayerState.Forest.BirdCount >= 3 &&
        PlayerState.Forest.HasBrownPowers == true
    then
        PlayerState.addFact("STRONG_FOOD_ENGINE");
        log("Strategic conclusion: Player has established a strong Forest food engine.");
}

rule "DeriveWetlandsCardCycling" {
    when
        PlayerState.Wetlands.BirdCount >= 2 &&
        PlayerState.HasFact("HAS_TUCK_BIRD")
    then
        PlayerState.addFact("WETLANDS_CYCLING_VIABLE");
        log("Strategic conclusion: Wetlands card cycling/tucking strategy is highly viable.");
}

rule "DeriveEggSpamStrategy" {
    when
        GameState.StrategicPhase == "LATE" &&
        PlayerState.Grassland.BirdCount >= 4
    then
        PlayerState.addFact("EGG_SPAM_DOMINANCE");
        log("Strategic conclusion: Grassland is highly optimized for late game egg spamming.");
}

rule "AnalyzeSTierBirdInHand" {
    when
        PlayerState.HandCards.contains(Bird.Tier == "S") &&
        GameState.StrategicPhase != "LATE"
    then
        PlayerState.addFact("PLAY_S_TIER_PRIORITY");
        log("Strategic conclusion: Found an S-Tier bird in hand during build phase. Prioritize playing immediately.");
}

rule "AnalyzeHighTierBirdInHand" {
    when
        PlayerState.HandCards.contains(Bird.Tier == "A") &&
        GameState.StrategicPhase != "LATE"
    then
        PlayerState.addFact("PLAY_HIGH_TIER_PRIORITY");
        log("Strategic conclusion: A-Tier bird in hand during build phase. High deployment priority.");
}

rule "AnalyzeGreenBoardGoalPush" {
    when
        GameState.GoalBoardSide == "GREEN" &&
        GameState.RemainingTurnsInRound <= 3 &&
        GameState.CurrentRoundGoal.OpponentMaxProgress - PlayerState.CurrentGoalProgress == 1
    then
        PlayerState.addFact("EOR_GOAL_PUSH");
        log("Strategic conclusion: Green board majority is contestable. One item away from tying or taking lead. Push goal.");
}

rule "AnalyzeGreenBoardGoalAbandon" {
    when
        GameState.GoalBoardSide == "GREEN" &&
        GameState.CurrentRoundGoal.OpponentMaxProgress - PlayerState.CurrentGoalProgress > GameState.RemainingTurnsInRound
    then
        PlayerState.addFact("EOR_GOAL_ABANDON");
        log("Strategic conclusion: Opponent lead on Green board is mathematically secure. Abandon EoR goal.");
}

rule "AnalyzeBlueBoardGoalMaxed" {
    when
        GameState.GoalBoardSide == "BLUE" &&
        PlayerState.CurrentGoalProgress >= 5
    then
        PlayerState.addFact("EOR_GOAL_ABANDON");
        log("Strategic conclusion: Blue board goal is already maximized at 5 points. Cease goal-focused actions.");
}

rule "AnalyzeBlueBoardGoalPush" {
    when
        GameState.GoalBoardSide == "BLUE" &&
        PlayerState.CurrentGoalProgress < 5 &&
        GameState.RemainingTurnsInRound <= 2
    then
        PlayerState.addFact("EOR_GOAL_PUSH");
        log("Strategic conclusion: Blue board is not maxed out and round is ending. Incremental points are guaranteed. Push goal.");
}

rule "AnalyzeFutureRoundGoalFeasibility" {
    when
        let futureGoal = GameState.GetFutureRoundGoal(1);
        let itemsInHandOrBoard = accumulate(
            Bird.MatchesCriteria(futureGoal.Criteria)
        );
        itemsInHandOrBoard >= 3 && GameState.StrategicPhase == "MID"
    then
        PlayerState.addFact("FUTURE_GOAL_PREPARED");
        log("Strategic conclusion: Current assets naturally align with the upcoming round goal. Retain these assets.");
}
```

#### Nivo 4 - Akumulacija prioriteta

Pravila ovog nivoa sabiru doprinose iz prethodnih zaključaka i dodeljuju numeričke prioritete svakoj akciji korišćenjem `accumulate` operatora.

```
rule "ScoreEggActionLateGame" {
    when
        GameState.StrategicPhase == "LATE" &&
        ActionEvaluation.ActionType == "LAY_EGGS" &&
        ActionEvaluation.IsValid == true
    then
        let maxAvailableSpace = accumulate(
            Bird.MaxEggs - Bird.CurrentEggs
        );
        ActionEvaluation.addScore(maxAvailableSpace * 2.0);
        ActionEvaluation.addTag("EGG_SPAM");
        ActionEvaluation.addReason("Maximizing raw victory points per turn using high-capacity grassland slots.");
        log("Scored LAY_EGGS action for late game strategy.");
}

rule "ScoreLayEggsMidGame" {
    when
        GameState.StrategicPhase == "MID" &&
        ActionEvaluation.ActionType == "LAY_EGGS" &&
        ActionEvaluation.IsValid == true &&
        PlayerState.Grassland.BirdCount >= 3
    then
        let availableSpace = accumulate(
            Bird.MaxEggs - Bird.CurrentEggs
        );
        ActionEvaluation.addScore(availableSpace * 1.2);
        ActionEvaluation.addTag("EGG_ENGINE_MID");
        ActionEvaluation.addReason("Grassland egg production in mid-game fuels future bird plays and positions for late-game scoring.");
        log("Scored LAY_EGGS action for mid-game grassland engine.");
}

rule "ScorePlayBirdEngine" {
    when
        GameState.StrategicPhase != "LATE" &&
        ActionEvaluation.ActionType == "PLAY_BIRD" &&
        ActionEvaluation.IsValid == true
    then
        let engineValue = accumulate(
            ActionEvaluation.TargetBird.TriggersOnActivation == true ? 5.0 :
            ActionEvaluation.TargetBird.HasPinkPower == true ? 3.5 : 1.0
        );
        ActionEvaluation.addScore(engineValue + ActionEvaluation.TargetBird.VictoryPoints);
        ActionEvaluation.addTag("ENGINE_BUILDING");
        ActionEvaluation.addReason("Playing an activation or passive power bird to compound engine value during the build phase.");
        log("Scored PLAY_BIRD action for engine potential.");
}

rule "ScoreDrawCardsWetlands" {
    when
        ActionEvaluation.ActionType == "DRAW_CARDS" &&
        ActionEvaluation.IsValid == true &&
        PlayerState.hasFact("WETLANDS_CYCLING_VIABLE")
    then
        let wetlandsDepth = accumulate(
            PlayerState.Wetlands.Birds.count(Bird.HasBrownOrPinkPower == true)
        );
        ActionEvaluation.addScore(wetlandsDepth * 3.5);
        ActionEvaluation.addTag("WETLANDS_ENGINE");
        ActionEvaluation.addReason("Wetlands card draw engine is active. Drawing cards maximizes tucking and cycling value per activation.");
        log("Scored DRAW_CARDS action for active wetlands engine.");
}

rule "ScoreGainFoodForest" {
    when
        ActionEvaluation.ActionType == "GAIN_FOOD" &&
        ActionEvaluation.IsValid == true &&
        PlayerState.hasFact("STRONG_FOOD_ENGINE")
    then
        let forestDepth = accumulate(
            PlayerState.Forest.Birds.count(Bird.HasBrownPower == true)
        );
        ActionEvaluation.addScore(forestDepth * 3.0);
        ActionEvaluation.addTag("FOREST_ENGINE");
        ActionEvaluation.addReason("Forest food engine is active. Gaining food triggers cascading brown power activations.");
        log("Scored GAIN_FOOD action for active forest engine.");
}

rule "ScorePlayBirdBonusContribution" {
    when
        ActionEvaluation.ActionType == "PLAY_BIRD" &&
        ActionEvaluation.IsValid == true &&
        ActionEvaluation.ContributesToBonus(PlayerState.ActiveBonusCards) == true
    then
        let bonusProgress = accumulate(
            PlayerState.ActiveBonusCards.progressAfterPlay(ActionEvaluation.TargetBird)
        );
        ActionEvaluation.addScore(bonusProgress * 3.0);
        ActionEvaluation.addTag("BONUS_CARD_PROGRESS");
        ActionEvaluation.addReason("Playing this bird advances bonus card completion, contributing to end-game point scoring.");
        log("Scored PLAY_BIRD action for bonus card contribution.");
}

rule "ScoreSTierBirdsHigh" {
    when
        ActionEvaluation.ActionType == "PLAY_BIRD" &&
        ActionEvaluation.IsValid == true &&
        ActionEvaluation.TargetBird.Tier == "S" &&
        !PlayerState.hasFact("EOR_GOAL_ABANDON")
    then
        ActionEvaluation.addScore(18.0);
        ActionEvaluation.addTag("WINGSPLAIN_META_S_TIER");
        ActionEvaluation.addReason("S-Tier birds provide outsized tempo advantages and can drive runaway wins. Deployment is a strong priority.");
        log("Applied S-Tier meta score weight.");
}

rule "ScoreATierBirds" {
    when
        ActionEvaluation.ActionType == "PLAY_BIRD" &&
        ActionEvaluation.IsValid == true &&
        ActionEvaluation.TargetBird.Tier == "A"
    then
        ActionEvaluation.addScore(12.0);
        ActionEvaluation.addTag("WINGSPLAIN_META_A_TIER");
        ActionEvaluation.addReason("A-Tier birds offer strong board presence and consistent value. High deployment priority.");
        log("Applied A-Tier meta score.");
}

rule "ScoreBTierBirds" {
    when
        ActionEvaluation.ActionType == "PLAY_BIRD" &&
        ActionEvaluation.IsValid == true &&
        ActionEvaluation.TargetBird.Tier == "B"
    then
        ActionEvaluation.addScore(6.0);
        ActionEvaluation.addTag("WINGSPLAIN_META_B_TIER");
        ActionEvaluation.addReason("B-Tier birds are effective and efficient. Solid bread-and-butter contributions to any engine.");
        log("Applied B-Tier meta score.");
}

rule "ScoreCTierBirdsContextual" {
    when
        ActionEvaluation.ActionType == "PLAY_BIRD" &&
        ActionEvaluation.IsValid == true &&
        ActionEvaluation.TargetBird.Tier == "C" &&
        (ActionEvaluation.ContributesToBonus(PlayerState.ActiveBonusCards) == true ||
         ActionEvaluation.ContributesToGoal(GameState.CurrentRoundGoal) == true)
    then
        ActionEvaluation.addScore(4.0);
        ActionEvaluation.addTag("WINGSPLAIN_META_C_TIER_CONTEXTUAL");
        ActionEvaluation.addReason("C-Tier bird gains contextual value by contributing to an active bonus card or round goal. Play is justified.");
        log("Applied C-Tier contextual score for bonus/goal alignment.");
}

rule "ScoreUnusedTierBirdsLow" {
    when
        ActionEvaluation.ActionType == "PLAY_BIRD" &&
        ActionEvaluation.IsValid == true &&
        ActionEvaluation.TargetBird.Tier == "UNDERUSED" &&
        ActionEvaluation.ContributesToBonus(PlayerState.ActiveBonusCards) == false &&
        ActionEvaluation.ContributesToGoal(GameState.CurrentRoundGoal) == false
    then
        ActionEvaluation.addScore(-5.0);
        ActionEvaluation.addTag("WINGSPLAIN_META_UNDERUSED_TIER");
        ActionEvaluation.addReason("Underused-tier birds consistently lose head-to-head decisions against better alternatives when no bonus or goal synergy exists.");
        log("Reduced priority score for Underused-tier bird without contextual justification.");
}

rule "ScoreApplyEoRGoalPush" {
    when
        PlayerState.hasFact("EOR_GOAL_PUSH") &&
        ActionEvaluation.IsValid == true &&
        ActionEvaluation.ContributesToGoal(GameState.CurrentRoundGoal) == true
    then
        ActionEvaluation.addScore(15.0);
        ActionEvaluation.addTag("EOR_GOAL_EFFICIENCY");
        ActionEvaluation.addReason("Action advances end-of-round goal standing in a critical micro-window.");
        log("Injected priority points into action for EoR goal push.");
}

rule "ScoreApplyEoRGoalAbandon" {
    when
        PlayerState.hasFact("EOR_GOAL_ABANDON") &&
        ActionEvaluation.IsValid == true &&
        ActionEvaluation.ContributesToGoal(GameState.CurrentRoundGoal) == true
    then
        ActionEvaluation.addScore(-12.0);
        ActionEvaluation.addTag("EOR_GOAL_EFFICIENCY");
        ActionEvaluation.addReason("Pursuing this goal yields dead tempo. Redirect actions to core engine or pure point extraction.");
        log("Penalized action that redundantly targets an abandoned or maxed EoR goal.");
}
```

### _Backward Chaining_

_Backward chaining_ se koristi za **ciljno orijentisano rezonovanje** - sistem polazi od pitanja i rekurzivno proverava uslove potrebne da se cilj dokaže.

Koristi se za dve klase upita:

1. **Validacija preporuke** - Da li je preporučena akcija zaista optimalna?
2. **Bonus karta analiza** - Koji je najkraći put do kompletiranja bonus karte?

#### Definisanje upita

```
query "IsBirdPlayOptimal" (Bird targetBird, Habitat targetHabitat) {
    Goal: ActionRecommendation(ActionType == "PLAY_BIRD", Bird == targetBird) :-
        prove: HasRequiredResources(targetBird, targetHabitat) &&
        prove: OutperformsAlternativeActions(targetBird) &&
        prove: StrategicallyAligned(targetBird);

    HasRequiredResources(b, h) :-
        PlayerState.FoodTokens.matchesCost(b.FoodCost) &&
        PlayerState.AvailableEggs >= h.RequiredEggsForNextSlot;

    OutperformsAlternativeActions(b) :-
        GameState.StrategicPhase == "EARLY" && (b.Tier == "S" || b.Tier == "A");

    OutperformsAlternativeActions(b) :-
        GameState.StrategicPhase == "MID" && (b.Tier == "S" || b.Tier == "A" || b.HasBrownOrPinkPower == true);

    OutperformsAlternativeActions(b) :-
        GameState.StrategicPhase == "LATE" && b.VictoryPoints >= 5;

    StrategicallyAligned(b) :-
        PlayerState.ActiveBonusCards.matchesCriteria(b) ||
        GameState.CurrentRoundGoal.matchesCriteria(b);
}

query "FindMinimalActionChainForBonus" (BonusCard bonusCard) {
    Goal: CompleteBonusCard(Card == bonusCard) :-
        prove: IdentifyMissingRequirements(bonusCard, out missingList) &&
        prove: GenerateOptimalSteps(missingList, out actionChain);

    IdentifyMissingRequirements(bc, missing) :-
        let missingCount = bc.RequiredTargetCount - PlayerState.MatchingBirdsCount(bc.Criteria) &&
        missing = new MissingRequirementSpecification(bc.Criteria, missingCount);

    GenerateOptimalSteps(m, chain) :-
        m.Count == 1 && PlayerState.HandCards.containsCriteria(m.Criteria) :-
        chain.addStep("PLAY_BIRD matching criteria from hand");

    GenerateOptimalSteps(m, chain) :-
        m.Count == 1 && !PlayerState.HandCards.containsCriteria(m.Criteria) :-
        chain.addStep("DRAW_CARDS from tray or deck to find matching bird") &&
        chain.addStep("PLAY_BIRD matching criteria");
}

query "EvaluateGoalViability" (EndOfRoundGoal goal) {
    Goal: TargetGoalViable(CurrentGoal == goal) :-
        prove: CalculateTurnsNeeded(goal, out turnsNeeded) &&
        prove: VerifyTurnAvailability(turnsNeeded) &&
        prove: CheckPayoffValue(goal);

    CalculateTurnsNeeded(g, t) :-
        let diff = g.OpponentMaxProgress - PlayerState.CurrentGoalProgress &&
        t = diff * 2;

    VerifyTurnAvailability(t) :-
        GameState.RemainingTurnsInRound >= t;

    CheckPayoffValue(g) :-
        GameState.GoalBoardSide == "GREEN";

    CheckPayoffValue(g) :-
        GameState.GoalBoardSide == "BLUE" && PlayerState.CurrentGoalProgress < 5;
}
```

#### Pravila koja koriste _Backward Chaining_

```
rule "InvokeBackwardChainingForHighTierValidation" {
    when
        GameState.StrategicPhase != "LATE" &&
        (PlayerState.HandCards.contains(Bird.Tier == "S") ||
         PlayerState.HandCards.contains(Bird.Tier == "A"))
    then
        let targetBird = PlayerState.HandCards.findBestByTier();
        let targetHabitat = targetBird.PreferredHabitat;

        if (prove IsBirdPlayOptimal(targetBird, targetHabitat)) {
            ActionEvaluation.addScore(20.0);
            ActionEvaluation.addTag("BC_VALIDATED_HIGH_TIER");
            ActionEvaluation.addReason("Backward chaining confirms high-tier bird play is optimal given current resources, strategic alignment, and alternatives.");
            log("Backward Chaining proved high-tier play is optimal. Added priority score.");
        }
}

rule "InvokeBackwardChainingForGoalAbandonment" {
    when
        GameState.GoalBoardSide == "GREEN" &&
        GameState.RemainingTurnsInRound <= 4
    then
        if (!prove EvaluateGoalViability(GameState.CurrentRoundGoal)) {
            PlayerState.addFact("EOR_GOAL_ABANDON");
            log("Backward Chaining disproved EoR goal viability. Added EOR_GOAL_ABANDON fact.");
        }
}
```

#### _Inference_ stablo

```
├── [GOAL]: ActionRecommendation(ActionType == "PLAY_BIRD", Bird == targetBird) [IsBirdPlayOptimal]
│   ├── [PROVE]: HasRequiredResources(targetBird, targetHabitat)
│   │   ├── [FACT]: PlayerState.FoodTokens.matchesCost(targetBird.FoodCost)
│   │   └── [FACT]: PlayerState.AvailableEggs >= targetHabitat.RequiredEggsForNextSlot
│   ├── [PROVE]: OutperformsAlternativeActions(targetBird)
│   │   ├── [MATCH]: GameState.StrategicPhase == "EARLY" && (targetBird.Tier == "S" || targetBird.Tier == "A")
│   │   ├── [MATCH]: GameState.StrategicPhase == "MID" && (targetBird.Tier == "S" || targetBird.Tier == "A" || targetBird.HasBrownOrPinkPower == true)
│   │   └── [MATCH]: GameState.StrategicPhase == "LATE" && targetBird.VictoryPoints >= 5
│   └── [PROVE]: StrategicallyAligned(targetBird)
│       ├── [MATCH]: PlayerState.ActiveBonusCards.matchesCriteria(targetBird)
│       └── [MATCH]: GameState.CurrentRoundGoal.matchesCriteria(targetBird)
│
├── [GOAL]: CompleteBonusCard(Card == bonusCard) [FindMinimalActionChainForBonus]
│   ├── [PROVE]: IdentifyMissingRequirements(bonusCard, out missingList)
│   │   └── [CALC]: missingCount = bc.RequiredTargetCount - PlayerState.MatchingBirdsCount(bc.Criteria)
│   └── [PROVE]: GenerateOptimalSteps(missingList, out actionChain)
│       ├── [BRANCH]: m.Count == 1 && PlayerState.HandCards.containsCriteria(m.Criteria)
│       │   └── [ACTION]: chain.addStep("PLAY_BIRD matching criteria from hand")
│       └── [BRANCH]: m.Count == 1 && !PlayerState.HandCards.containsCriteria(m.Criteria)
│           ├── [ACTION]: chain.addStep("DRAW_CARDS from tray or deck to find matching bird")
│           └── [ACTION]: chain.addStep("PLAY_BIRD matching criteria")
│
└── [GOAL]: TargetGoalViable(CurrentGoal == goal) [EvaluateGoalViability]
    ├── [PROVE]: CalculateTurnsNeeded(goal, out turnsNeeded)
    │   └── [CALC]: diff = goal.OpponentMaxProgress - PlayerState.CurrentGoalProgress -> turnsNeeded = diff * 2
    ├── [PROVE]: VerifyTurnAvailability(turnsNeeded)
    │   └── [FACT]: GameState.RemainingTurnsInRound >= turnsNeeded
    └── [PROVE]: CheckPayoffValue(goal)
        ├── [MATCH]: GameState.GoalBoardSide == "GREEN"
        └── [MATCH]: GameState.GoalBoardSide == "BLUE" && PlayerState.CurrentGoalProgress < 5
```

### CEP - _Complex Event Processing_

Sistem prati tok partije kroz vremenski uređenu sekvencu događaja. Svaki odigrani potez generiše `TurnEvent` koji se ubacuje u radnu memoriju sa vremenskom oznakom. CEP mehanizam detektuje obrasce u ovim sekvencama i generiše nove strateške činjenice.

CEP događaji su domenski relevantni - ne prate tehnička stanja sistema već **strateška stanja partije**.

#### Pravila za detekciju CEP obrazaca i emitovanje činjenica

```
rule "CEPDetectOpponentFoodMonopoly" {
    when
        let foodEvents = collectSequence(
            TurnEvent.ActionType == "GAIN_FOOD" over LastTurns(4)
        );
        foodEvents.all(PlayerId != GameState.CurrentPlayerId) &&
        GameState.BirdFeeder.ContainsDiceType("Rodent") == false
    then
        GameState.addFact("OPPONENT_FOOD_MONOPOLY_RODENT");
        log("CEP Pattern Triggered: Opponents heavily draining specific food resource. High starvation threat.");
}

rule "CEPDetectTempoAcceleration" {
    when
        let highValuePlays = collectSequence(
            TurnEvent.ActionType == "PLAY_BIRD" && TurnEvent.BirdPoints >= 6 over LastTurns(3)
        );
        highValuePlays.Count >= 2
    then
        GameState.addFact("OPPONENT_TEMPO_ACCELERATING");
        log("CEP Pattern Triggered: High point birds are being chained rapidly by opponents. Shift to high value yields needed.");
}

rule "CEPDetectAggressiveEggSpam" {
    when
        let eggSpamEvents = collectSequence(
            TurnEvent.ActionType == "LAY_EGGS" over LastTurns(3)
        );
        eggSpamEvents.all(PlayerId != GameState.CurrentPlayerId) &&
        GameState.CurrentRound == 4
    then
        GameState.addFact("OPPONENT_EGG_SPAM_CRITICAL");
        log("CEP Pattern Triggered: Opponents are maximizing standard egg conversions. Pure point maximization required to stay competitive.");
}
```

#### Pravila koja koriste CEP činjenice u evaluaciji akcija

```
rule "RespondToOpponentFoodMonopoly" {
    when
        GameState.hasFact("OPPONENT_FOOD_MONOPOLY_RODENT") &&
        ActionEvaluation.ActionType == "PLAY_BIRD" &&
        ActionEvaluation.TargetBird.FoodCost.contains("Rodent")
    then
        ActionEvaluation.addScore(-15.0);
        ActionEvaluation.addTag("TEMPO_RISK");
        ActionEvaluation.addReason("CEP indicators reveal heavy competition over Rodent food tokens. Playing this bird now will stall your engine.");
        log("Adjusted score down for rodent-costing bird due to CEP food monopoly fact.");
}

rule "RespondToOpponentTempoAcceleration" {
    when
        GameState.hasFact("OPPONENT_TEMPO_ACCELERATING") &&
        ActionEvaluation.ActionType == "PLAY_BIRD" &&
        (ActionEvaluation.TargetBird.Tier == "S" || ActionEvaluation.TargetBird.Tier == "A")
    then
        ActionEvaluation.addScore(15.0);
        ActionEvaluation.addTag("COUNTER_TEMPO");
        ActionEvaluation.addReason("Opponents are pushing points aggressively. Deploying a high-tier response bird is the best way to offset their point trajectory.");
        log("Boosted high-tier bird deployment priority in response to opponent tempo acceleration.");
}

rule "RespondToCriticalEggSpam" {
    when
        GameState.hasFact("OPPONENT_EGG_SPAM_CRITICAL") &&
        ActionEvaluation.ActionType == "LAY_EGGS" &&
        ActionEvaluation.IsValid == true
    then
        ActionEvaluation.addScore(10.0);
        ActionEvaluation.addTag("FORCE_POINT_PARITY");
        ActionEvaluation.addReason("CEP confirms standard late game egg rush by table. Matching egg volume is required to secure placement.");
        log("Boosted egg actions to maintain point parity with opponents.");
}
```

## Evolucija baze znanja

Nakon svakog poteza:

- stanje igre se ažurira,
- nova činjenica se dodaje u radnu memoriju,
- istorija događaja se čuva i
- sistem može analizirati trendove tokom partije.

# Interakcije zasnovane na znanju

Proces rada sistema zasniva se na ciklusu rezonovanja tipičnom za sisteme bazirane na znanju.

Sistem funkcioniše u dva moda koji odgovaraju fazama igre.

**Tok u fazi draftovanja:**

1. Igrač inicijalizuje stanje partije i unosi izvučene karte ptica i bonus karte
2. Ulazni podaci transformišu se u skup činjenica i ubacuju u radnu memoriju
3. Aktiviraju se pravila faze draftovanja
4. Sistem generiše preporuke: koje karte zadržati i zašto

**Tok u glavnoj fazi:**

1. Igrač unosi trenutno stanje partije
2. Ulazni podaci transformišu se u skup činjenica i ubacuju u radnu memoriju
3. CEP mehanizam analizira sekvencu prethodnih događaja i generiše kontekstualne činjenice
4. Pokreće se _forward chaining_ kroz četiri nivoa
5. _Backward chaining_ proverava validnost izabrane preporuke i analizira minimalni lanac akcija za bonus karte
6. Sistem vraća rangirane odluke igraču

Na ovaj način sistem ne donosi odluku direktnim izračunavanjem, već kroz proces postepenog zaključivanja nad bazom znanja.

# Primer rezonovanja za glavnu fazu (korak po korak)

Sledeći primer ilustruje način na koji sistem dolazi do preporuke poteza.

## Scenario

Partija je u sledećem stanju:

- Runda 3, preostala su **3 poteza** (**strateška faza: MID**);
- Aktivni cilj runde: **najviše ptica u šumi** (zelena strana table, _majority_ bodovanje);
- Igrač ima **dve ptice u šumi** (_slot_ 1 i 2 zauzeti, obe ptice imaju braon moći), vodeći protivnik ima **tri** ptice u šumi;
- _Slot_ 3 u šumi je slobodan, zahteva **2 jajeta**;
- Akcija "Uzmi hranu" u šumi aktivira obe postavljene ptice i uzimanje 2 kocke iz _birdfeeder_-a;
- Tabla igrača:
  - **Šuma**: 2 ptice (obe sa braon moćima - generisanje hrane pri aktivaciji)
  - **Livada**: 3 ptice (ukupan slobodan kapacitet jaja: 5)
  - **Močvara**: 2 ptice (jedna sa _tuck_ sposobnošću)
- Igrač poseduje:
  - **1 jaje**,
  - **1 pšenicu** i
  - **1 bobicu**;
- Bonus karta igrača: **_Anatomist_** (boduje se broj ptica sa delom tela u imenu: glava, krilo, grudi, noga, rep... Igrač trenutno ima 3 takve ptice, a sledeći prag je 4 ptice za 7 bodova);
- U ruci igrača se nalazi ptica **Rose-breasted Grosbeak** vredna **6 bodova**, sa **braon moći** (uzimanje bobice iz hranilice), igra se isključivo u šumi, zahteva **1 crva, 1 pšenicu i 1 bobicu**;
- U _bird-feeder_-u su kockice: miš, crv, pšenica, bobica, bobica.

## Korak 1 - Ubacivanje činjenica

Ulazni podaci iz unosa igrača transformišu se u atomske činjenice i ubacuju u radnu memoriju sistema:

```
GameState.CurrentRound = 3
GameState.RemainingTurnsInRound = 3
GameState.GoalBoardSide = "GREEN"
GameState.CurrentRoundGoal.Type = "FOREST_BIRD_COUNT"
GameState.CurrentRoundGoal.OpponentMaxProgress = 3
GameState.BirdFeeder = { RODENT: 1, WORM: 1, WHEAT: 1, BERRY: 2 }
GameState.CardTray.HasCards = true

PlayerState.CurrentGoalProgress = 2
PlayerState.TotalBirdsPlayed = 7
PlayerState.TotalRemainingEggCapacity = 6

PlayerState.Forest.BirdCount = 2
PlayerState.Forest.HasBrownPowers = true
PlayerState.Forest.Slot3.RequiredEggs = 2

PlayerState.Grassland.BirdCount = 3
PlayerState.Grassland.TotalRemainingEggCapacity = 5

PlayerState.Wetlands.BirdCount = 2
PlayerState.HasFact("HAS_TUCK_BIRD") = true

PlayerState.FoodTokens = { WHEAT: 1, BERRY: 1 }
PlayerState.AvailableEggs = 1

PlayerState.HandCards = [
  Bird {
    Name: "Rose-breasted Grosbeak",
    Tier: "U",
    VictoryPoints: 6,
    HasBrownPower: true,
    TriggersOnActivation: true,
    FoodCost: { WORM: 1, WHEAT: 1, BERRY: 1 },
    Habitat: FOREST,
    MatchesBonusCard: true
  }
]

PlayerState.ActiveBonusCards = [
  BonusCard {
    Name: "Anatomist",
    Criteria: "BODY_PART_IN_NAME",
    RequiredTargetCount: 4,
    CurrentMatchingCount: 3
  }
]
```

Sistem sada poseduje potpunu sliku stanja i može pokrenuti rezonovanje.

## Korak 2 - Nivo 1: detekcija strateške faze

Sistem evaluje sva tri pravila za detekciju faze. Jedino `DetectMidPhase` ispunjava uslov:

```
DetectEarlyPhase:
  GameState.CurrentRound == 1 -> FALSE
  (GameState.CurrentRound == 2 && TotalBirdsPlayed <= 3) -> FALSE
  -> ne okida

DetectMidPhase:
  (GameState.CurrentRound == 2 && TotalBirdsPlayed > 3) -> FALSE
  GameState.CurrentRound == 3 -> TRUE
  -> OKIDA

DetectLatePhase:
  GameState.CurrentRound == 4 -> FALSE
  -> ne okida
```

Zaključak se upisuje u radnu memoriju:

```
GameState.StrategicPhase = "MID"
```

## Korak 3 - Nivo 2: validnost akcija

Sistem prolazi kroz svaku moguću akciju i proverava uslove validnosti:

**`PLAY_BIRD` (ptica iz ruke -> šuma, _slot_ 3)**

```
ValidatePlayBird:
  PlayerState.HandCards.contains(targetBird) -> TRUE
  PlayerState.FoodTokens.matchesCost({ WORM: 1, WHEAT: 1, BERRY: 1 }) -> 0 WORM < 1 -> FALSE
  PlayerState.AvailableEggs >= Forest.Slot3.RequiredEggs -> 1 < 2 -> FALSE
  -> AKCIJA NIJE VALIDNA (nedostaje 1 crv i 1 jaje)
```

**`LAY_EGGS`**

```
ValidateLayEggs:
  PlayerState.TotalRemainingEggCapacity > 0 -> 6 > 0 -> TRUE
  -> VALIDNA
```

**`GAIN_FOOD`**

```
ValidateGainFood:
  GameState.BirdFeeder.HasDice == true -> TRUE
  -> VALIDNA
```

**`DRAW_CARDS`**

```
ValidateDrawCards:
  GameState.CardTray.HasCards == true -> TRUE
  -> VALIDNA
```

Validne akcije za dalju evaluaciju: `LAY_EGGS`, `GAIN_FOOD`, `DRAW_CARDS`.

## Korak 4 - Nivo 3: strateški zaključci

Na osnovu validacionih zaključaka i stanja igre, Nivo 3 izvodi apstraktne strateške činjenice:

**`DeriveWetlandsCardCycling`**

```
PlayerState.Wetlands.BirdCount >= 2 -> TRUE
PlayerState.HasFact("HAS_TUCK_BIRD") -> TRUE
-> PlayerState.addFact("WETLANDS_CYCLING_VIABLE")
```

**`AnalyzeGreenBoardGoalPush`**

```
GameState.GoalBoardSide == "GREEN" -> TRUE
GameState.RemainingTurnsInRound <= 3 -> 3 <= 3 -> TRUE
OpponentMaxProgress - PlayerState.CurrentGoalProgress == 1 -> 3 - 2 = 1 == 1 -> TRUE  -> PlayerState.addFact("EOR_GOAL_PUSH")
```

## Korak 5 - Nivo 4: akumulacija prioriteta

Sistem dodeljuje numeričke skorove svakoj validnoj akciji primenom svih relevantnih pravila Nivoa 4.

**`LAY_EGGS`**

| Pravilo                 | Uslov                                                                            | Doprinos         |
| ----------------------- | -------------------------------------------------------------------------------- | ---------------- |
| `ScoreLayEggsMidGame`   | MID faza , 3 ptice na livadi , slobodan kapacitet = 5                            | `5 × 1.2 = +6.0` |
| `ScoreApplyEoRGoalPush` | `EOR_GOAL_PUSH` aktivan, ali `LAY_EGGS` ne doprinosi broju ptica u šumi direktno | `0`              |

**Ukupan skor `LAY_EGGS`: 6.0** - Tagovi: `EGG_ENGINE_MID`, `ENABLER`

**`GAIN_FOOD`**

| Pravilo               | Uslov                                                         | Doprinos         |
| --------------------- | ------------------------------------------------------------- | ---------------- |
| `ScoreGainFoodForest` | Engine aktivan, igraču nedostaje resurs (crv) za pticu u ruci | `2 × 2.0 = +4.0` |

**Ukupan skor `GAIN_FOOD`: 4.0** - Tagovi: `RESOURCE_FIX`

**`DRAW_CARDS`**

| Pravilo                  | Uslov                                                    | Doprinos         |
| ------------------------ | -------------------------------------------------------- | ---------------- |
| `ScoreDrawCardsWetlands` | `WETLANDS_CYCLING_VIABLE` , 1 braon/roza ptica u močvari | `1 × 3.5 = +3.5` |

**Ukupan skor `DRAW_CARDS`: 3.5** - Tagovi: `WETLANDS_ENGINE`

**Projekcija sledećeg poteza (informativno):** `PLAY_BIRD` bi akumulirao:

| Pravilo                                                                                  | Doprinos |
| ---------------------------------------------------------------------------------------- | -------- |
| `ScorePlayBirdEngine` (MID faza, braon moć -> 5.0, + 6 VP)                               | `+11.0`  |
| `ScorePlayBirdBonusContribution` (_Anatomist_: prelazak na prag od 4 ptice donosi +4 VP) | `+4.0`   |
| `ScoreApplyEoRGoalPush` (ptica ide u šumu -> direktno doprinosi cilju runde)             | `+15.0`  |
| **Ukupno**                                                                               | **30.0** |

## Korak 6 - Validacija (_Backward Chaining_)

Sistem proverava cilj pozivanjem `InvokeBackwardChainingForGoalAbandonment`:

```
EvaluateGoalViability(FOREST_BIRD_COUNT_GOAL)

  [PROVE] CalculateTurnsNeeded(goal, out turnsNeeded):
    diff = 1 (zaostaje 1 pticu)
    neededItems = { EGGS: 1, WORM: 1 }
    turnsNeeded = 3
      (potez 1: GAIN_FOOD za crva, potez 2: LAY_EGGS za jaje, potez 3: PLAY_BIRD)
    -> SUCCESS (3 <= 3 preostala poteza)

  [PROVE] VerifyTurnAvailability(turnsNeeded = 3):
    GameState.RemainingTurnsInRound >= 3 -> TRUE
    -> SUCCESS
```

## Korak 7 - Razumljiv izlaz sistema

Igrač dobija listu rangiranih preporuka sa obrazloženjem:

> **Preporuka #1: Položi jaja (`LAY_EGGS`)**
>
> **Skor:** 6.0 | **Tagovi:** `EGG_ENGINE_MID`, `ENABLER`
>
> **Obrazloženje:** Nedostaje **1 jaje** za slot 3 u šumi. _Engine_ u livadi (3 ptice) u MID fazi je primarni izvor VP i resursa. Potez omogućava igranje Rose-breasted Grosbeak u narednih 2 poteza.

> **Preporuka #2: Uzmi hranu (`GAIN_FOOD`)**
>
> **Skor:** 4.0 | **Tagovi:** `RESOURCE_FIX`
>
> **Obrazloženje:** Nedostaje **1 crv** za Rose-breasted Grosbeak. Resurs je dostupan u hranilici.

> **Preporuka #3: Vuci karte (`DRAW_CARDS`)**
>
> **Skor:** 3.5 | **Tagovi:** `WETLANDS_ENGINE`
>
> **Obrazloženje:** Močvarni engine (2 ptice) je operativan. Potez ne rešava trenutni nedostatak resursa (jaje/crv) niti doprinosi `EOR_GOAL_PUSH`.

Sistem nije doneo odluku direktno - rangirao je sve validne akcije na osnovu višestepenog zaključivanja i prikazao igraču obrazložen izbor.

# Arhitektura sistema

Predloženi sistem biće realizovan kao višeslojna aplikacija koja razdvaja korisnički interfejs, domensku logiku i mehanizam zaključivanja.

Korisnički interfejs biće razvijen korišćenjem **Angular** radnog okvira i služiće za unos trenutnog stanja partije, prikaz preporučenih akcija i obrazloženja procesa rezonovanja.

Centralna logika sistema biće implementirana u programskom jeziku **Rust** zbog svojih visokih performansi, memorijske bezbednosti i bezbednosti tipova. Rust je pogodan za implementaciju rule-based sistema zbog biblioteke [rust-rule-engine](https://github.com/KSD-CO/rust-rule-engine), koja nudi sve neophodne funkcionalnosti potrebne za realizaciju opisanog mehanizma zaključivanja.

Aplikacija će biti realizovana kao desktop i mobilna Android aplikacija. Za povezivanje Angular interfejsa i Rust logike koristiće se radni okvir [Tauri](https://tauri.app/), koji omogućava izradu _lightweight_ _cross-platform_ aplikacija koje dele isti _codebase_.
