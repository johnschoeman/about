echo "---- Start netlify.sh"

echo "---- Install trunk"
brew install trunk

echo "---- Install tailwindcss"
brew install tailwindcss

echo "---- Build app"
trunk build --release
