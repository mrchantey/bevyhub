set windows-shell := ["C:/tools/cygwin/bin/sh.exe","-c"]
set dotenv-load


default:
	just --list --unsorted

init-repo:
	just assets-pull
	just export-scenes

app *scenes:
	cargo run --example bevyhub_app -- {{scenes}}

api:
	cd crates/bevyhub_api && just run

app-terminal:
	just app \
	scenes/camera-2d.json \
	scenes/ui-terminal-input.json \

app-space:
	just app \
	scenes/camera-2d.json \
	scenes/space-scene.json	\

run example *args:
	cargo run --example {{example}} {{args}}

cli *args:
	cargo run -p bevyhub-cli -- {{args}}

export-scenes *args:
	cargo run --example export_scenes {{args}}
	cargo run -p bevyhub_scene --example export_test_scene {{args}}
	cd crates/bevyhub_template && cargo run --example export_scenes {{args}}

ts-dst:= '../bevyhub-site/packages/editor/src/serdeTypes/generated'

export-typescript *args:
	cargo run --example export_typescript
	rm -rf {{ts-dst}} || true
	mkdir -p {{ts-dst}}
	cp -r target/typescript/* {{ts-dst}}

install-cli *args:
	cargo install --path ./crates/cli {{args}}

build-wasm *args:
	@echo "🚀 exporting bevyhub"
	just export-scenes
	bevyhub build \
	--example bevyhub_app \
	--release \
	--copy-local ../bevyhub-apps \
	--copy-scenes scenes \
	--copy-registries target/registries {{args}}
	@echo "🚀 exporting bevyhub_template"
	cd crates/bevyhub_template && just export-scenes
	bevyhub build \
	-p bevyhub_template --example app \
	--release \
	--copy-local ../bevyhub-apps \
	--copy-scenes crates/bevyhub_template/scenes \
	--copy-registries crates/bevyhub_template/target/registries {{args}}

# use this to verify changes to the cli are working
build-wasm-test *args:
	just cli build \
	-p bevyhub_template --example bevyhub_app \
	--release	\
	--copy-local ../bevyhub-apps \
	--copy-scenes crates/bevyhub_template/scenes \
	--copy-registries target/registries \
	{{args}}

export-test-scene:
	cargo run -p bevyhub_scene --example export_test_scene

test crate *args:
	just watch 'cargo test -p {{crate}} --lib --all-features -- {{args}}'

test-all *args:
	cargo test -p bevyhub_core --all-features
	cargo test -p bevyhub_net --all-features
	cargo test -p bevyhub_scene --all-features
	cd crates/bevyhub_api && just test-all

assets-push:
	aws s3 sync ./assets s3://bevyhub-public/assets --delete
	tar -czvf ./assets.tar.gz ./assets
	aws s3 cp ./assets.tar.gz s3://bevyhub-public/assets.tar.gz
	rm ./assets.tar.gz

assets-pull:
	curl -o ./assets.tar.gz https://bevyhub-public.s3.us-west-2.amazonaws.com/assets.tar.gz
	tar -xzvf ./assets.tar.gz
	rm ./assets.tar.gz

publish crate *args:
	cargo publish -p {{crate}} --allow-dirty --no-verify {{args}}
	sleep 1

publish-all *args:
	just publish bevyhub_scene 	 	{{args}} 	|| true
	just publish bevyhub_net 		 	{{args}} 	|| true
	just publish bevyhub_core 		{{args}} 	|| true
	just publish bevyhub 				 	{{args}} 	|| true
	just publish bevyhub_template {{args}}	|| true
	just publish bevyhub_api 		 	{{args}}	|| true
	just publish bevyhub-cli 		 	{{args}}	|| true
# just publish bevyhub_server 	 {{args}} || true


patch:
	cargo set-version --bump patch

watch *command:
	forky watch \
	-w '**/*.rs' \
	-i '{.git,target,html}/**' \
	-i '**/mod.rs' \
	-- {{command}}