watch-spicetify:
	spicetify watch -s

watch-compiler:
	watchexec -w "themes" "just build" 

watch:
	just watch-compiler & just watch-spicetify 

compile:
	./compiler/target/debug/compiler

build-compiler:
	cd compiler; cargo b

build:
	just build-compiler; just compile
