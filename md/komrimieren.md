## 1. `tar` – Tape Archive

`tar` erstellt Archive und kann sie direkt komprimieren (gzip, bzip2, xz, zstd …).

### Wichtige Optionen

| Option | Bedeutung |
|--------|-----------|
| `-c`   | Create (Archiv erstellen) |
| `-x`   | Extract (entpacken) |
| `-t`   | List (Inhalt anzeigen) |
| `-f`   | File (Archivdatei angeben) |
| `-v`   | Verbose (ausführliche Ausgabe) |
| `-z`   | gzip-Kompression (`.tar.gz` / `.tgz`) |
| `-j`   | bzip2-Kompression (`.tar.bz2`) |
| `-J`   | xz-Kompression (`.tar.xz`) |
| `-a`   | Kompression anhand der Dateiendung erkennen |
| `-C`   | In Verzeichnis wechseln |

### Archive erstellen

```bash
# Unkomprimiertes tar-Archiv
tar -cvf archiv.tar ordner/ datei.txt

# gzip (sehr gängig)
tar -czvf archiv.tar.gz ordner/

# bzip2 (höhere Kompression, langsamer)
tar -cjvf archiv.tar.bz2 ordner/

# xz (noch höhere Kompression)
tar -cJvf archiv.tar.xz ordner/

# zstd (modern, schnell & gut)
tar --zstd -cvf archiv.tar.zst ordner/

# Kompression automatisch an Endung erkennen
tar -cavf archiv.tar.gz ordner/
```

### Archive entpacken

```bash
# .tar
tar -xvf archiv.tar

# .tar.gz / .tgz
tar -xzvf archiv.tar.gz

# .tar.bz2
tar -xjvf archiv.tar.bz2

# .tar.xz
tar -xJvf archiv.tar.xz

# .tar.zst
tar --zstd -xvf archiv.tar.zst

# Automatisch anhand der Endung
tar -xavf archiv.tar.gz

# In ein bestimmtes Verzeichnis entpacken
tar -xzvf archiv.tar.gz -C /ziel/verzeichnis/
```

### Inhalt anzeigen (ohne entpacken)

```bash
tar -tzvf archiv.tar.gz
tar -tvf archiv.tar
```

### Einzelne Dateien/Ordner aus Archiv holen

```bash
tar -xzvf archiv.tar.gz pfad/zur/datei.txt
tar -xzvf archiv.tar.gz ordner/unterordner/
```

### Dateien zu bestehendem Archiv hinzufügen

```bash
# Nur bei unkomprimierten .tar-Archiven zuverlässig
tar -rvf archiv.tar neue_datei.txt
```

### Fortschritt anzeigen (mit `pv`)

```bash
tar -czf - ordner/ | pv > archiv.tar.gz
```

---

## 2. `zip` / `unzip` – ZIP-Archive

ZIP ist weit verbreitet und plattformübergreifend (Windows, macOS, Linux).

### Installation (falls nicht vorhanden)

```bash
# Debian/Ubuntu
sudo apt install zip unzip

# Fedora
sudo dnf install zip unzip

# Arch
sudo pacman -S zip unzip
```

### ZIP-Archiv erstellen

```bash
# Einzelne Dateien
zip archiv.zip datei1.txt datei2.txt

# Ganzes Verzeichnis rekursiv
zip -r archiv.zip ordner/

# Mit maximaler Kompression (Stufe 9)
zip -r -9 archiv.zip ordner/

# Bestimmte Dateien ausschließen
zip -r archiv.zip ordner/ -x "*.tmp" -x "*.log"

# Versteckt/Symbolische Links mitnehmen etc. (je nach Bedarf)
zip -r archiv.zip ordner/
```

### ZIP entpacken

```bash
# In aktuelles Verzeichnis
unzip archiv.zip

# In bestimmtes Verzeichnis
unzip archiv.zip -d /ziel/verzeichnis/

# Vorhandene Dateien nicht überschreiben
unzip -n archiv.zip

# Immer überschreiben (ohne Nachfrage)
unzip -o archiv.zip

# Nur bestimmte Dateien
unzip archiv.zip datei.txt "ordner/*"
```

### Inhalt anzeigen

```bash
unzip -l archiv.zip
unzip -v archiv.zip   # etwas ausführlicher
```

### Dateien zu ZIP hinzufügen / aktualisieren

```bash
zip archiv.zip neue_datei.txt
zip -u archiv.zip geaenderte_datei.txt   # update
```

### Passwortschutz

```bash
# Mit Passwort erstellen (Eingabe wird versteckt)
zip -r -e archiv_geschuetzt.zip ordner/

# Oder Passwort direkt übergeben (unsicherer – landet in History)
zip -r -P geheim123 archiv.zip ordner/
```

---

## 3. Schneller Vergleich

| Merkmal              | `tar` + Kompression      | `zip`                     |
|----------------------|--------------------------|---------------------------|
| Typische Endungen    | `.tar.gz`, `.tar.xz` …  | `.zip`                    |
| Unix-Metadaten       | Sehr gut (Rechte, Owner) | Begrenzt                  |
| Einzeldatei-Zugriff  | Schwächer                | Gut                       |
| Mehrere Volumes      | Möglich                  | Möglich                   |
| Passwort             | Nur mit zusätzlichen Tools (z. B. `gpg`) | Eingebaut (`-e`) |
| Plattform            | Linux/Unix Standard      | Überall (Windows-freundlich) |
| Empfohlen für        | Backups, Source-Code, Server | Austausch mit Windows-Usern |

---

## 4. Praktische Tipps & Best Practices

```bash
# 1. Große Verzeichnisse: xz oder zstd für bessere Rate
tar -cJvf backup-$(date +%F).tar.xz /wichtige/daten/

# 2. Schnell & gut → zstd
tar --zstd -cvf backup.tar.zst ordner/

# 3. Nur bestimmte Dateitypen packen
find . -name "*.pdf" | zip -@ dokumente.zip
tar -czvf logs.tar.gz --exclude='*.tmp' logs/

# 4. Integrität prüfen
gzip -t archiv.tar.gz          # Test gzip
xz -t archiv.tar.xz            # Test xz
unzip -t archiv.zip            # Test zip

# 5. Mit gpg verschlüsseln (tar)
tar -czf - ordner/ | gpg -c > archiv.tar.gz.gpg
gpg -d archiv.tar.gz.gpg | tar -xz

# 6. Parallel komprimieren (pigz, pbzip2, pxz)
tar -cf - ordner/ | pigz > archiv.tar.gz
tar -cf - ordner/ | pxz > archiv.tar.xz
```

### Nützliche Aliase (optional in `~/.bashrc`)

```bash
alias tgz='tar -czvf'
alias untgz='tar -xzvf'
alias tj='tar -cjvf'
alias tx='tar -cJvf'
```

---

## 5. Häufige Fehler vermeiden

- Immer `-f` bei `tar` verwenden und den **Archivnamen direkt danach** schreiben.
- Bei `tar` die Kompressionsoption (`-z`, `-J` …) **vor** `-f` setzen (Reihenfolge der Kurzoptionen ist meist egal, aber klarer).
- Große Archive nicht unkomprimiert lassen, wenn Speicherplatz knapp ist.
- Bei ZIP unter Linux: Rechte/Ownership gehen teilweise verloren → für System-Backups lieber `tar` nutzen.
- Symbolische Links: `tar` behält sie standardmäßig; bei `zip` ggf. `-y` beachten.

---

## Kurz-Cheat-Sheet

```bash
# tar.gz erstellen & entpacken
tar -czvf name.tar.gz ordner/
tar -xzvf name.tar.gz

# tar.xz
tar -cJvf name.tar.xz ordner/
tar -xJvf name.tar.xz

# zip
zip -r name.zip ordner/
unzip name.zip
```


