SET "websrc = .\src-tauri\websrc"
IF EXIST websrc (
    RD /S /Q %websrc%
) ELSE (
    ECHO "No %websrc% to remove"
)
MKDIR %websrc%

CD ".\cube_shuffle-wasm"
trunk build --release --dist ".\..\src-tauri\websrc"