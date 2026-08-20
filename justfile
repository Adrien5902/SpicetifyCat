watch-spicetify:
	spicetify watch -s

watch-compiler:
	watchexec -w "themes" -w "compiler" -- "just build" 

alias w := watch
watch:
	just watch-compiler & just watch-spicetify 

compile:
	./compiler/target/debug/compiler

build-compiler:
	cd compiler; cargo b

alias b := build
build:
	just build-compiler; just compile
