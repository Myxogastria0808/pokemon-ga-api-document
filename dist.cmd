@echo off

trunk build --release --public-url pokemon-ga-api-document
cd dist
git init
git add .
git commit -m "release"
git branch gh-pages
git switch gh-pages
git remote add origin https://github.com/Myxogastria0808/pokemon-ga-api-document.git
git push -f origin gh-pages
cd ..