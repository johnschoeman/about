echo "Start netlify.sh"

echo "Install trunk"
brew install trunk

echo "Build app"
trunk build --release
