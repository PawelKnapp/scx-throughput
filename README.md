# SCX Throughput Scheduler

Szkielet planisty CPU wykorzystujący podsystem `sched-ext` oraz bibliotekę `scx_rustland_core`.

## ⚠️ Ważna informacja o uruchamianiu
Biblioteka `scx_rustland_core` próbuje w locie kompilować kod eBPF, co wymaga obecności zmiennych środowiskowych `OUT_DIR` oraz `CARGO_MANIFEST_DIR` w sesji roota. Uruchomienie samej binarki (`sudo ./scx_throughput`) zakończy się błędem `environment variable not found`.

## Jak uruchomić?

1. Zbuduj projekt standardowo:
   ```bash
   cargo build
   ```

2. Uruchom używając załączonego skryptu (który wstrzykuje wymagane zmienne dla wbudowanego kompilatora):
   ```bash
   ./run.sh
   ```

*(Skrypt wykona pod spodem: `sudo env OUT_DIR="/tmp" CARGO_MANIFEST_DIR="$(pwd)" ./target/debug/scx_throughput`)*