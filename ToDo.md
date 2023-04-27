- [ ] Liste Tage des aktuellen Monates (eine Woche je Zeile, mit Wochennummer). Wochenanfang ist Montag
- [ ] Markiere "Heute"
- [ ] Mit Eingabe eines Monates diesen Monat anzeigen
- [ ] Mit Eingabe eines Jahres + Monates diesen Monat anzeigen
- [ ] Tag + Monat eingeben um Termin hinzuzufügen
- [ ] Termin hat Überschrift/Titel + mehrzeilige Notiz
- [ ] In Datei speichern. Format ist beliebig, empfehle eine Textdatei mit Datum als Namen pro Termin.
- [ ] Bei Programstart wieder laden + Tage mit Terminen hervorheben
- [ ] Auf Eingabe eines bestimmten Kommandos alle Termine in der Zukunft auflisten
- [ ] Durchnummerieren und auf Eingabe der Nummer den Text dazu anzeigen
- [ ] Termin löschbar machen
- [ ] Einen Kommandozeilenparameter akzeptieren, der das Anzeigen eines Menüs unterdrückt und eine Mail als Reminder sendet, wenn "Morgen" ein Termin ansteht (damit man es als cron laufen lassen könnte)!
- [ ] Exportiere beliebigen Monat zu statischem HTML. Ohne "heute" aber mit makierten Terminen und mit einer List unter dem Kalender mit den Überschriften der Termine + Datum
- [ ] GUI/TUI bauen
- [ ] read ics files and display the events it contains
- [ ] write tests

```
 Montag | Dienstag | Mittwoch | Donnerstag | Freitag | Samstag | Sonntag
--------|----------|----------|------------|---------|---------|----------
 -      | -        | 1.       | 2.         | 3.      | 4.      | 5.
 6.     | 7.       | 8.       | 9.         | 10.     | 11.     | 12.
 13.    | 14.      | 15.      | 16.        | 17.     | 18.     | 19.
 20.    | 21.      | 22.      | 23.        | 24.     | 25.     | 26.
 27.    | 28.      | -        | -          | -       | -       | - 

 Montag | Dienstag | Mittwoch | Donnerstag | Freitag | Samstag | Sonntag
--------|----------|----------|------------|---------|---------|----------
 -      | -        | 1        | 2          | 3       | 4       | 5
 6      | 7        | 8        | 9          | 10      | 11      | 12
 13     | 14       | 15       | 16         | 17      | 18      | 19
 20     | 21       | 22       | 23         | 24      | 25      | 26
 27     | 28       | 29       | 30         | 31      | -       | -

 Montag | Dienstag | Mittwoch | Donnerstag | Freitag | Samstag | Sonntag
--------|----------|----------|------------|---------|---------|----------
 -      | -        | -        | -          | -       | 1       | 2
 3      | 4        | 5        | 6          | 7       | 8       | 9
 10     | 11       | 12       | 13         | 14      | 15      | 16
 17     | 18       | 19       | 20         | 21      | 22      | 23
 24     | 25       | 26       | 27         | 28      | 29      | 30

 Januar 2023
 Montag | Dienstag | Mittwoch | Donnerstag | Freitag | Samstag | Sonntag
--------|----------|----------|------------|---------|---------|----------
 -      | -        | -        | -          | -       | -       | 1
 2      | 3        | 4        | 5          | 6       | 7       | 8
 9      | 10       | 11       | 12         | 13      | 14      | 15
 16     | 17       | 18       | 19         | 20      | 21      | 22
 23     | 24       | 25       | 26         | 27      | 28      | 29
 30     | 31       | -        | -          | -       | -       | -

 März 2023
 Montag | Dienstag | Mittwoch | Donnerstag | Freitag | Samstag | Sonntag
--------|----------|----------|------------|---------|---------|---------
 -      | -        | 1        | 2          | 3       | 4       | 5
 6      | 7        | 8        | 9          | 10      | 11      | 12
 13     | 14       | 15       | 16         | 17      | 18      | 19
 20     | 21       | 22       | 23         | 24      | 25      | 26
 27     | 28       | 29       | 30         | 31      | -       | -
```
- Gucken was der erste und letze Tag im Monat für ein Wochentag sind und dann bis zur vollen Woche mit dummy values auffüllen

- struct für jeden Wochentag und dann jedes Datum dem passenden Wochentag zuordnen und dann nacheinander ausgeben