* zdjęcia
    * IMG_001
    * IMG_002
    * ... 
    * IMG_9001

```
$ ustrukturyzuj --pomóż(((
./ustrukturyzuj [opcje]

Options:
    -k, --katalog ~/nieposortowane-zdjęcia
                        Katalog na którym będziem pracować(domyślnie ".")
    -r, --rekureku      rekurencyjnie zaglębiaj się w podkatalogi
    -p, --pomóż(((      wypisz ten tekst
$ ./ustrukturyzuj
```
* zdjęcia
     * 1994-07-01
            * IMG_001
            * ... 
            * IMG_024
     * 1994-07-02
            * IMG_025
            * ... 
            * IMG_326
     * ...
        
### DOZRO
 * sprawdzanie czy nie nadpisuje plików
 * obsługa błędów
 * rozdzielanie względem przedziału czasu innym niż dzień
 * możliwość zmiany wyglądu daty w nazwie folderu
 * rozdzielanie plików nie tylko po dacie (np po regekspie nazwy)