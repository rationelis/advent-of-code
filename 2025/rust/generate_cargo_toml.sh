OUTPUT_FILE="Cargo.toml"

echo "[package]" > $OUTPUT_FILE
echo 'name = "advent_of_code"' >> $OUTPUT_FILE
echo 'version = "0.1.0"' >> $OUTPUT_FILE
echo 'edition = "2021"' >> $OUTPUT_FILE
echo "" >> $OUTPUT_FILE

DAY_FILES=$(ls src/day*.rs 2>/dev/null)

for FILE in $DAY_FILES; do
    MODULE_NAME=$(basename "$FILE" .rs)
    echo "[[bin]]" >> $OUTPUT_FILE
    echo "name = \"$MODULE_NAME\"" >> $OUTPUT_FILE
    echo "path = \"src/$MODULE_NAME.rs\"" >> $OUTPUT_FILE
    echo "" >> $OUTPUT_FILE
done
