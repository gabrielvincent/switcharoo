# Changelog

## [0.8.2](https://github.com/H3rmt/hyprshell-test/compare/v0.8.1...v0.8.2) (2025-05-30)


### Bug Fixes

* background loading of icons ([f5c8ff0](https://github.com/H3rmt/hyprshell-test/commit/f5c8ff0e29713432726bb841bafd8dd6729331fa))
* fix focus window problem ([3c702a1](https://github.com/H3rmt/hyprshell-test/commit/3c702a1da63d56168aa6e7ba2932842c55f97c8d))
* fix slow start times ([0c4bb95](https://github.com/H3rmt/hyprshell-test/commit/0c4bb9585806de61f73e885026083e49b2fe2048))
* fix switch mode monitor select ([b8c1d44](https://github.com/H3rmt/hyprshell-test/commit/b8c1d440d47fe4a6ca2c1690e1b0be65b81de1df))
* Update Nix Package ([a147d38](https://github.com/H3rmt/hyprshell-test/commit/a147d385ee5928e0616d232f741657069242272f))

## [0.8.1](https://github.com/H3rmt/hyprshell-test/compare/v0.8.0...v0.8.1) (2025-05-29)


### Bug Fixes

* fix ci publish ([bb1e659](https://github.com/H3rmt/hyprshell-test/commit/bb1e65987ac7c371b29cbf09000bb4f117f5b0e9))

## [0.8.0](https://github.com/H3rmt/hyprshell-test/compare/v0.7.2...v0.8.0) (2025-05-29)


### Features

* Add NixOS `home-manager` module ([cd20717](https://github.com/H3rmt/hyprshell-test/commit/cd207178a0cfd44c7ad1069880ef35532f5547ae))
* added config file migrations ([db2f6cd](https://github.com/H3rmt/hyprshell-test/commit/db2f6cd9fb3c08ab2f9858fb9c1dac61540353b5))
* added custom args for hyprshell systemd ([aa01139](https://github.com/H3rmt/hyprshell-test/commit/aa01139aebfe2dcd717b670ac6ce557f93c2f1d0))
* better debug commands ([b17a393](https://github.com/H3rmt/hyprshell-test/commit/b17a393b04201beab8b582a340d1bb80bef5cda2))


### Bug Fixes

* better icon detection ([453888e](https://github.com/H3rmt/hyprshell-test/commit/453888e61551946cfa3dec92409df606f7aa04db))
* check for config file extensions at start ([7a41bb2](https://github.com/H3rmt/hyprshell-test/commit/7a41bb2bf8abea7b2bc2efc2544deb26243faf7c))
* don't unset all CSS styles at the start, only necessary ([b42d25c](https://github.com/H3rmt/hyprshell-test/commit/b42d25c1d342d10a5af378e1ebcae167a83ca01f))
* fix multiple run week caches not being added together ([c027209](https://github.com/H3rmt/hyprshell-test/commit/c027209d6f01ec10210420182ea3283380f8e74f))
* fix overview click on client or workspace ([71d445c](https://github.com/H3rmt/hyprshell-test/commit/71d445c0f310f986b769eb420b55e21e9559d94d))
* fix wrong name in hm module ([99386d5](https://github.com/H3rmt/hyprshell-test/commit/99386d56c0e40e65bc491e2b87cf28ef83305f6d))
* force command now accepts args ([7a41bb2](https://github.com/H3rmt/hyprshell-test/commit/7a41bb2bf8abea7b2bc2efc2544deb26243faf7c))
* nix allow source and text attributes ([dcd9e41](https://github.com/H3rmt/hyprshell-test/commit/dcd9e417071e80cfd489eaefc8908a6b12a324eb))
* nix json config fixes ([ced4a45](https://github.com/H3rmt/hyprshell-test/commit/ced4a45ef6b8361663b30062298f3084f2219a37))
* **nix:** Fix HM Module ([b8367b4](https://github.com/H3rmt/hyprshell-test/commit/b8367b4de9dcd2a5b6e49a3e9322534657de4886))
* open windows earlier ([b42d25c](https://github.com/H3rmt/hyprshell-test/commit/b42d25c1d342d10a5af378e1ebcae167a83ca01f))
* remove socat dependency ([01758a6](https://github.com/H3rmt/hyprshell-test/commit/01758a6d6384c5ad73a841c3f2e8b90ee9912393))
* search for installed terminals from PATH ([453888e](https://github.com/H3rmt/hyprshell-test/commit/453888e61551946cfa3dec92409df606f7aa04db))
* some focus fixes ([9fa8009](https://github.com/H3rmt/hyprshell-test/commit/9fa80093f943f6941aeaeb424a262cb3b4c40ec6))
* sort launcher applications by shorted exec instead of full (removed /bin/flatpak...) ([b17a393](https://github.com/H3rmt/hyprshell-test/commit/b17a393b04201beab8b582a340d1bb80bef5cda2))
* try all config extensions if file missing ([6cd4799](https://github.com/H3rmt/hyprshell-test/commit/6cd4799198c7b170537135a611c8ed88b97aa62f))
* use char instead of String for key for websearch plugins ([eba4282](https://github.com/H3rmt/hyprshell-test/commit/eba42823569cbf19dcb35cf37cc78db7bcdb0e3b))

## [0.7.2](https://github.com/H3rmt/hyprshell-test/compare/v0.7.1...v0.7.2) (2025-05-13)


### Bug Fixes

* ci release ([4a0f45f](https://github.com/H3rmt/hyprshell-test/commit/4a0f45f466d47a12e6cab70d2e71c835287287a0))

## [0.7.1](https://github.com/H3rmt/hyprshell-test/compare/v0.7.0...v0.7.1) (2025-05-13)


### Bug Fixes

* arrow keys in the launcher ([179ca7b](https://github.com/H3rmt/hyprshell-test/commit/179ca7b45e865875584a4c658a0455ea00af0bc6))
* speedup animation ([179ca7b](https://github.com/H3rmt/hyprshell-test/commit/179ca7b45e865875584a4c658a0455ea00af0bc6))

## [0.7.0](https://github.com/H3rmt/hyprshell-test/compare/v0.6.3...v0.7.0) (2025-05-10)


### Features

* added `data` command to see LaunchHistory ([8e3de53](https://github.com/H3rmt/hyprshell-test/commit/8e3de53b31834c8a034d28d26d72ebcbbd4d9815))


### Bug Fixes

* reload desktop maps on close ([204358d](https://github.com/H3rmt/hyprshell-test/commit/204358dc20ad49ff44d79444f707d20a4535d0da))
* remove size_factor from config ([77b53bd](https://github.com/H3rmt/hyprshell-test/commit/77b53bd205d32c0619d244a601cea8304f4a4b9c))
* update documentation ([229921d](https://github.com/H3rmt/hyprshell-test/commit/229921d82167d59670cdeab488677b372ecafe73))

## [0.6.3](https://github.com/H3rmt/hyprshell-test/compare/v0.5.4...v0.6.3) (2025-05-10)


### Features

* add animation to plugin close launch ([b944274](https://github.com/H3rmt/hyprshell-test/commit/b9442742b5a357966061c66e594b9104d158fe7b))
* add calc plugin ([06c8a41](https://github.com/H3rmt/hyprshell-test/commit/06c8a41db42d482b777f158fcf9eb1b23708cc13))
* add debug command ([00afa1e](https://github.com/H3rmt/hyprshell-test/commit/00afa1e34b9716a041d3bd33734700836075bd70))
* add run shell commands from launcher ([879cbba](https://github.com/H3rmt/hyprshell-test/commit/879cbba0597281c14b07dadfe03149b4323caf6f))
* add websearch plugin ([1a079d1](https://github.com/H3rmt/hyprshell-test/commit/1a079d1552036f8713ba69f33271475ff4a41103))
* added click on clients and workspaces in overview and switch ([328fc3b](https://github.com/H3rmt/hyprshell-test/commit/328fc3b432b28ad1e390be10f945e1425708d430))
* added systemd generation (use --no-systemd to disable) ([97c2c7f](https://github.com/H3rmt/hyprshell-test/commit/97c2c7f88c3ba221863a43b8adbfb50b444fa841))
* click on entry in launcher works ([87076ef](https://github.com/H3rmt/hyprshell-test/commit/87076ef8c715fc0f7e29a7c2187aa59f17514df5))
* NixOS Support ([#171](https://github.com/H3rmt/hyprshell-test/issues/171)) ([d42f12b](https://github.com/H3rmt/hyprshell-test/commit/d42f12be62c08d3764bc91b034bc7aa05d531608))
* rewrite hyprswitch ([198cd0f](https://github.com/H3rmt/hyprshell-test/commit/198cd0f5ae03210b46cfaba8dbf5f8d30fcc77a9))
* rewrite hyprswitch ([0d834ab](https://github.com/H3rmt/hyprshell-test/commit/0d834ab64cb607d2bc10ca7b13d2642728592f50))


### Bug Fixes

* add nix back ([9efadcd](https://github.com/H3rmt/hyprshell-test/commit/9efadcdc37ae0c639ab36a1880e3d48b1b6c51a2))
* allow hold of tab and arrow keys to switch ([e0aaa57](https://github.com/H3rmt/hyprshell-test/commit/e0aaa570f54fce0155631c5720805886a0c275a1))
* close launcher on esc ([a003b82](https://github.com/H3rmt/hyprshell-test/commit/a003b8227b709a7bb186e2b3703ec70f3f16dd7d))
* detect socat path at runtime ([312f667](https://github.com/H3rmt/hyprshell-test/commit/312f6677165a023466a115d8a61f2927b31adc71))
* don't exit launcher when no items ([a4356b7](https://github.com/H3rmt/hyprshell-test/commit/a4356b783c148f61736fcd9e4af23d63b1be7c85))
* filter apps in the launcher by name, exec and details ([ab0f23e](https://github.com/H3rmt/hyprshell-test/commit/ab0f23e43c4ce540e14fe3f0ac515fa52ec56ab9))
* fix ci release ([e498ca1](https://github.com/H3rmt/hyprshell-test/commit/e498ca19f86fcf2d4668fa7582f9320232fd194d))
* fix ci release ([6dc00c7](https://github.com/H3rmt/hyprshell-test/commit/6dc00c719ef7beaea70c86f7851c6de8cdf9117e))
* Fix Nix Build ([02bd2c4](https://github.com/H3rmt/hyprshell-test/commit/02bd2c400616cb96cd21ccc6b2f530143fcb511b))
* fix overflow for selecting workspaces ([9efadcd](https://github.com/H3rmt/hyprshell-test/commit/9efadcdc37ae0c639ab36a1880e3d48b1b6c51a2))
* fix panic when listening for changes on nonexisting file ([2e02509](https://github.com/H3rmt/hyprshell-test/commit/2e025099b4b259844a6f5192a4e9db3db10a00ba))
* fix right alt bind ([4394240](https://github.com/H3rmt/hyprshell-test/commit/43942408b0febe18026e278d2e4cffd7eece25db))
* fix versions ([899b2cc](https://github.com/H3rmt/hyprshell-test/commit/899b2cc95018a342c2bf45d0fdf0ef971616be7b))
* fix versions ([e9565fd](https://github.com/H3rmt/hyprshell-test/commit/e9565fd627c85ed4fa8fcf26cd35c06ceeaeb2b7))
* fix versions ([8d9a5ee](https://github.com/H3rmt/hyprshell-test/commit/8d9a5eeeee5d74fef9501fec39c5d8692061ec97))
* fix versions ([05f3da6](https://github.com/H3rmt/hyprshell-test/commit/05f3da60197e59d6a882fa0282127ae62091f879))
* fix versions ([b4e0380](https://github.com/H3rmt/hyprshell-test/commit/b4e0380cced9b3ae35e3d8368fb336edc5530274))
* fix versions ([14e4a72](https://github.com/H3rmt/hyprshell-test/commit/14e4a725dfe6b0d70c235db80069238338ec2890))
* fix versions ([89711a6](https://github.com/H3rmt/hyprshell-test/commit/89711a628c45d6286742ac274ad4ec2fe487faed))
* get socat path at buildtime ([a4356b7](https://github.com/H3rmt/hyprshell-test/commit/a4356b783c148f61736fcd9e4af23d63b1be7c85))
* make css optional ([b0c36ee](https://github.com/H3rmt/hyprshell-test/commit/b0c36eeb3f9dfbe9a85933505dc618cd0c231308))
* move scripts ([97329b7](https://github.com/H3rmt/hyprshell-test/commit/97329b723d4c90e674fa74d65fb4397252266b85))
* moved size_factor to scale for a more sensible default and bounds check ([a85f3b8](https://github.com/H3rmt/hyprshell-test/commit/a85f3b8948211711acf232b9834f4c9c2afadf61))
* show recent windows on one screen only ([f6a3016](https://github.com/H3rmt/hyprshell-test/commit/f6a301689827ebb27afc10d445715c070a5762f1))
* switch window if no results are present in launcher ([50a81a0](https://github.com/H3rmt/hyprshell-test/commit/50a81a0503f8f259374fdafab73e383f930902fe))
* try to fix publication to creates.io ([c911a07](https://github.com/H3rmt/hyprshell-test/commit/c911a078e40655fd869df317479a6a93cce508b2))


### Documentation

* fix css explain images ([d86d566](https://github.com/H3rmt/hyprshell-test/commit/d86d5667201031915100391c1eeb9571e763f370))


### Continuous Integration

* fix ci release ([a24657b](https://github.com/H3rmt/hyprshell-test/commit/a24657bb9e237e69ff2f8687577114c25de921fe))
* fix ci release ([e02df4a](https://github.com/H3rmt/hyprshell-test/commit/e02df4a193718664762ba8d7e22c63814f061de3))
* fix ci release ([d8d481d](https://github.com/H3rmt/hyprshell-test/commit/d8d481d672809a4f8907a156eed1705caa27a9aa))
* fix release-please again ([824bf03](https://github.com/H3rmt/hyprshell-test/commit/824bf032ab3b131121b16d9c16ce9f6a5215c580))
* fix release-please again ([bce2335](https://github.com/H3rmt/hyprshell-test/commit/bce23355cd3f477e95179acadd5d1401544b1822))
* fix releases ([06ad9fc](https://github.com/H3rmt/hyprshell-test/commit/06ad9fc8cc85a4a6fe3584510bce0efbd1aa1425))

## [0.5.4](https://github.com/H3rmt/hyprshell-test/compare/v0.5.3...v0.5.4) (2025-05-08)


### Continuous Integration

* fix ci release ([a24657b](https://github.com/H3rmt/hyprshell-test/commit/a24657bb9e237e69ff2f8687577114c25de921fe))

## [0.5.3](https://github.com/H3rmt/hyprshell-test/compare/v0.5.2...v0.5.3) (2025-05-08)


### Continuous Integration

* fix ci release ([e02df4a](https://github.com/H3rmt/hyprshell-test/commit/e02df4a193718664762ba8d7e22c63814f061de3))

## [0.5.2](https://github.com/H3rmt/hyprshell-test/compare/v0.5.1...v0.5.2) (2025-05-08)


### Bug Fixes

* make css optional ([b0c36ee](https://github.com/H3rmt/hyprshell-test/commit/b0c36eeb3f9dfbe9a85933505dc618cd0c231308))


### Continuous Integration

* fix ci release ([d8d481d](https://github.com/H3rmt/hyprshell-test/commit/d8d481d672809a4f8907a156eed1705caa27a9aa))

## [0.5.1](https://github.com/H3rmt/hyprshell-test/compare/v0.5.0...v0.5.1) (2025-05-08)


### Features

* add calc plugin ([06c8a41](https://github.com/H3rmt/hyprshell-test/commit/06c8a41db42d482b777f158fcf9eb1b23708cc13))
* add run shell commands from launcher ([879cbba](https://github.com/H3rmt/hyprshell-test/commit/879cbba0597281c14b07dadfe03149b4323caf6f))
* add websearch plugin ([1a079d1](https://github.com/H3rmt/hyprshell-test/commit/1a079d1552036f8713ba69f33271475ff4a41103))
* added click on clients and workspaces in overview and switch ([328fc3b](https://github.com/H3rmt/hyprshell-test/commit/328fc3b432b28ad1e390be10f945e1425708d430))
* added systemd generation (use --no-systemd to disable) ([97c2c7f](https://github.com/H3rmt/hyprshell-test/commit/97c2c7f88c3ba221863a43b8adbfb50b444fa841))
* click on entry in launcher works ([87076ef](https://github.com/H3rmt/hyprshell-test/commit/87076ef8c715fc0f7e29a7c2187aa59f17514df5))
* NixOS Support ([#171](https://github.com/H3rmt/hyprshell-test/issues/171)) ([d42f12b](https://github.com/H3rmt/hyprshell-test/commit/d42f12be62c08d3764bc91b034bc7aa05d531608))
* rewrite hyprswitch ([198cd0f](https://github.com/H3rmt/hyprshell-test/commit/198cd0f5ae03210b46cfaba8dbf5f8d30fcc77a9))
* rewrite hyprswitch ([0d834ab](https://github.com/H3rmt/hyprshell-test/commit/0d834ab64cb607d2bc10ca7b13d2642728592f50))


### Bug Fixes

* add nix back ([9efadcd](https://github.com/H3rmt/hyprshell-test/commit/9efadcdc37ae0c639ab36a1880e3d48b1b6c51a2))
* allow hold of tab and arrow keys to switch ([e0aaa57](https://github.com/H3rmt/hyprshell-test/commit/e0aaa570f54fce0155631c5720805886a0c275a1))
* close launcher on esc ([a003b82](https://github.com/H3rmt/hyprshell-test/commit/a003b8227b709a7bb186e2b3703ec70f3f16dd7d))
* detect socat path at runtime ([312f667](https://github.com/H3rmt/hyprshell-test/commit/312f6677165a023466a115d8a61f2927b31adc71))
* don't exit launcher when no items ([a4356b7](https://github.com/H3rmt/hyprshell-test/commit/a4356b783c148f61736fcd9e4af23d63b1be7c85))
* filter apps in the launcher by name, exec and details ([ab0f23e](https://github.com/H3rmt/hyprshell-test/commit/ab0f23e43c4ce540e14fe3f0ac515fa52ec56ab9))
* fix ci release ([e498ca1](https://github.com/H3rmt/hyprshell-test/commit/e498ca19f86fcf2d4668fa7582f9320232fd194d))
* fix ci release ([6dc00c7](https://github.com/H3rmt/hyprshell-test/commit/6dc00c719ef7beaea70c86f7851c6de8cdf9117e))
* Fix Nix Build ([02bd2c4](https://github.com/H3rmt/hyprshell-test/commit/02bd2c400616cb96cd21ccc6b2f530143fcb511b))
* fix overflow for selecting workspaces ([9efadcd](https://github.com/H3rmt/hyprshell-test/commit/9efadcdc37ae0c639ab36a1880e3d48b1b6c51a2))
* fix panic when listening for changes on nonexisting file ([2e02509](https://github.com/H3rmt/hyprshell-test/commit/2e025099b4b259844a6f5192a4e9db3db10a00ba))
* fix versions ([899b2cc](https://github.com/H3rmt/hyprshell-test/commit/899b2cc95018a342c2bf45d0fdf0ef971616be7b))
* fix versions ([e9565fd](https://github.com/H3rmt/hyprshell-test/commit/e9565fd627c85ed4fa8fcf26cd35c06ceeaeb2b7))
* fix versions ([8d9a5ee](https://github.com/H3rmt/hyprshell-test/commit/8d9a5eeeee5d74fef9501fec39c5d8692061ec97))
* fix versions ([05f3da6](https://github.com/H3rmt/hyprshell-test/commit/05f3da60197e59d6a882fa0282127ae62091f879))
* fix versions ([b4e0380](https://github.com/H3rmt/hyprshell-test/commit/b4e0380cced9b3ae35e3d8368fb336edc5530274))
* fix versions ([14e4a72](https://github.com/H3rmt/hyprshell-test/commit/14e4a725dfe6b0d70c235db80069238338ec2890))
* fix versions ([89711a6](https://github.com/H3rmt/hyprshell-test/commit/89711a628c45d6286742ac274ad4ec2fe487faed))
* get socat path at buildtime ([a4356b7](https://github.com/H3rmt/hyprshell-test/commit/a4356b783c148f61736fcd9e4af23d63b1be7c85))
* moved size_factor to scale for a more sensible default and bounds check ([a85f3b8](https://github.com/H3rmt/hyprshell-test/commit/a85f3b8948211711acf232b9834f4c9c2afadf61))
* show recent windows on one screen only ([f6a3016](https://github.com/H3rmt/hyprshell-test/commit/f6a301689827ebb27afc10d445715c070a5762f1))
* switch window if no results are present in launcher ([50a81a0](https://github.com/H3rmt/hyprshell-test/commit/50a81a0503f8f259374fdafab73e383f930902fe))
* try to fix publication to creates.io ([c911a07](https://github.com/H3rmt/hyprshell-test/commit/c911a078e40655fd869df317479a6a93cce508b2))


### Continuous Integration

* fix release-please again ([824bf03](https://github.com/H3rmt/hyprshell-test/commit/824bf032ab3b131121b16d9c16ce9f6a5215c580))
* fix release-please again ([bce2335](https://github.com/H3rmt/hyprshell-test/commit/bce23355cd3f477e95179acadd5d1401544b1822))
* fix releases ([06ad9fc](https://github.com/H3rmt/hyprshell-test/commit/06ad9fc8cc85a4a6fe3584510bce0efbd1aa1425))

## [0.4.2](https://github.com/H3rmt/hyprshell-test/compare/0.4.0...0.4.2) (2025-05-06)


### Bug Fixes

* test8 ([7e73810](https://github.com/H3rmt/hyprshell-test/commit/7e73810f90f511f575758e2360b4a03c7fa80e87))

## [0.4.0](https://github.com/H3rmt/hyprshell-test/compare/4.0.2...0.4.0) (2025-05-06)


### Features

* add calc plugin ([06c8a41](https://github.com/H3rmt/hyprshell-test/commit/06c8a41db42d482b777f158fcf9eb1b23708cc13))
* add run shell commands from launcher ([879cbba](https://github.com/H3rmt/hyprshell-test/commit/879cbba0597281c14b07dadfe03149b4323caf6f))
* add websearch plugin ([1a079d1](https://github.com/H3rmt/hyprshell-test/commit/1a079d1552036f8713ba69f33271475ff4a41103))
* added click on clients and workspaces in overview and switch ([328fc3b](https://github.com/H3rmt/hyprshell-test/commit/328fc3b432b28ad1e390be10f945e1425708d430))
* added systemd generation (use --no-systemd to disable) ([97c2c7f](https://github.com/H3rmt/hyprshell-test/commit/97c2c7f88c3ba221863a43b8adbfb50b444fa841))
* click on entry in launcher works ([87076ef](https://github.com/H3rmt/hyprshell-test/commit/87076ef8c715fc0f7e29a7c2187aa59f17514df5))
* NixOS Support ([#171](https://github.com/H3rmt/hyprshell-test/issues/171)) ([d42f12b](https://github.com/H3rmt/hyprshell-test/commit/d42f12be62c08d3764bc91b034bc7aa05d531608))
* rewrite hyprswitch ([198cd0f](https://github.com/H3rmt/hyprshell-test/commit/198cd0f5ae03210b46cfaba8dbf5f8d30fcc77a9))
* rewrite hyprswitch ([0d834ab](https://github.com/H3rmt/hyprshell-test/commit/0d834ab64cb607d2bc10ca7b13d2642728592f50))


### Bug Fixes

* add nix back ([9efadcd](https://github.com/H3rmt/hyprshell-test/commit/9efadcdc37ae0c639ab36a1880e3d48b1b6c51a2))
* allow hold of tab and arrow keys to switch ([e0aaa57](https://github.com/H3rmt/hyprshell-test/commit/e0aaa570f54fce0155631c5720805886a0c275a1))
* close launcher on esc ([a003b82](https://github.com/H3rmt/hyprshell-test/commit/a003b8227b709a7bb186e2b3703ec70f3f16dd7d))
* detect socat path at runtime ([312f667](https://github.com/H3rmt/hyprshell-test/commit/312f6677165a023466a115d8a61f2927b31adc71))
* don't exit launcher when no items ([a4356b7](https://github.com/H3rmt/hyprshell-test/commit/a4356b783c148f61736fcd9e4af23d63b1be7c85))
* filter apps in the launcher by name, exec and details ([79c7076](https://github.com/H3rmt/hyprshell-test/commit/79c7076758a20c81586b9956f1c5fa2fb5cc0094))
* fix ci release ([e498ca1](https://github.com/H3rmt/hyprshell-test/commit/e498ca19f86fcf2d4668fa7582f9320232fd194d))
* fix ci release ([6dc00c7](https://github.com/H3rmt/hyprshell-test/commit/6dc00c719ef7beaea70c86f7851c6de8cdf9117e))
* Fix Nix Build ([02bd2c4](https://github.com/H3rmt/hyprshell-test/commit/02bd2c400616cb96cd21ccc6b2f530143fcb511b))
* fix overflow for selecting workspaces ([9efadcd](https://github.com/H3rmt/hyprshell-test/commit/9efadcdc37ae0c639ab36a1880e3d48b1b6c51a2))
* fix panic when listening for changes on nonexisting file ([2e02509](https://github.com/H3rmt/hyprshell-test/commit/2e025099b4b259844a6f5192a4e9db3db10a00ba))
* fix versions ([899b2cc](https://github.com/H3rmt/hyprshell-test/commit/899b2cc95018a342c2bf45d0fdf0ef971616be7b))
* fix versions ([e9565fd](https://github.com/H3rmt/hyprshell-test/commit/e9565fd627c85ed4fa8fcf26cd35c06ceeaeb2b7))
* fix versions ([8d9a5ee](https://github.com/H3rmt/hyprshell-test/commit/8d9a5eeeee5d74fef9501fec39c5d8692061ec97))
* fix versions ([05f3da6](https://github.com/H3rmt/hyprshell-test/commit/05f3da60197e59d6a882fa0282127ae62091f879))
* fix versions ([b4e0380](https://github.com/H3rmt/hyprshell-test/commit/b4e0380cced9b3ae35e3d8368fb336edc5530274))
* fix versions ([14e4a72](https://github.com/H3rmt/hyprshell-test/commit/14e4a725dfe6b0d70c235db80069238338ec2890))
* fix versions ([89711a6](https://github.com/H3rmt/hyprshell-test/commit/89711a628c45d6286742ac274ad4ec2fe487faed))
* get socat path at buildtime ([a4356b7](https://github.com/H3rmt/hyprshell-test/commit/a4356b783c148f61736fcd9e4af23d63b1be7c85))
* moved size_factor to scale for a more sensible default and bounds check ([a85f3b8](https://github.com/H3rmt/hyprshell-test/commit/a85f3b8948211711acf232b9834f4c9c2afadf61))
* switch window if no results are present in launcher ([50a81a0](https://github.com/H3rmt/hyprshell-test/commit/50a81a0503f8f259374fdafab73e383f930902fe))
* test ([2079219](https://github.com/H3rmt/hyprshell-test/commit/2079219d7e51e827d4c8f544eac4319f4338d163))
* test2 ([134820a](https://github.com/H3rmt/hyprshell-test/commit/134820a039695bbe3b6ebf18669504cd30a40f29))
* test3 ([995909f](https://github.com/H3rmt/hyprshell-test/commit/995909fcf416d233e91554e761ceb21430150b6c))
* test4 ([35e9f5a](https://github.com/H3rmt/hyprshell-test/commit/35e9f5a657b499d6f90265c98804c491b0ab825a))
* test5 ([7f55743](https://github.com/H3rmt/hyprshell-test/commit/7f557437bafb7bb9cede1492e32d9fc899d31f5f))
* test5 ([949ac94](https://github.com/H3rmt/hyprshell-test/commit/949ac9472098ffdb2444289de2bb3b1dacd04774))
* test6 ([90c87cd](https://github.com/H3rmt/hyprshell-test/commit/90c87cdc538269e4e25b37aff614ca76dfa2caf3))
* test7 ([92eeecb](https://github.com/H3rmt/hyprshell-test/commit/92eeecb785fe7c45632d894ced66a3649f13d2ec))
* try to fix publication to creates.io ([c911a07](https://github.com/H3rmt/hyprshell-test/commit/c911a078e40655fd869df317479a6a93cce508b2))


### Continuous Integration

* fix release-please again ([824bf03](https://github.com/H3rmt/hyprshell-test/commit/824bf032ab3b131121b16d9c16ce9f6a5215c580))
* fix release-please again ([bce2335](https://github.com/H3rmt/hyprshell-test/commit/bce23355cd3f477e95179acadd5d1401544b1822))
* tests ([5b83cdc](https://github.com/H3rmt/hyprshell-test/commit/5b83cdcc2c86cdc68a674d76c537aa673ffbc403))

## [4.0.2](https://github.com/H3rmt/hyprshell-test/compare/4.0.1...4.0.2) (2025-05-06)


### Bug Fixes

* test6 ([90c87cd](https://github.com/H3rmt/hyprshell-test/commit/90c87cdc538269e4e25b37aff614ca76dfa2caf3))

## [4.0.1](https://github.com/H3rmt/hyprshell-test/compare/4.0.0...4.0.1) (2025-05-06)


### Bug Fixes

* test5 ([7f55743](https://github.com/H3rmt/hyprshell-test/commit/7f557437bafb7bb9cede1492e32d9fc899d31f5f))
* test5 ([949ac94](https://github.com/H3rmt/hyprshell-test/commit/949ac9472098ffdb2444289de2bb3b1dacd04774))

## [4.0.0](https://github.com/H3rmt/hyprshell-test/compare/4.0.0...4.0.0) (2025-05-06)


### Features

* add calc plugin ([06c8a41](https://github.com/H3rmt/hyprshell-test/commit/06c8a41db42d482b777f158fcf9eb1b23708cc13))
* add run shell commands from launcher ([879cbba](https://github.com/H3rmt/hyprshell-test/commit/879cbba0597281c14b07dadfe03149b4323caf6f))
* add websearch plugin ([1a079d1](https://github.com/H3rmt/hyprshell-test/commit/1a079d1552036f8713ba69f33271475ff4a41103))
* added click on clients and workspaces in overview and switch ([328fc3b](https://github.com/H3rmt/hyprshell-test/commit/328fc3b432b28ad1e390be10f945e1425708d430))
* added systemd generation (use --no-systemd to disable) ([97c2c7f](https://github.com/H3rmt/hyprshell-test/commit/97c2c7f88c3ba221863a43b8adbfb50b444fa841))
* click on entry in launcher works ([87076ef](https://github.com/H3rmt/hyprshell-test/commit/87076ef8c715fc0f7e29a7c2187aa59f17514df5))
* NixOS Support ([#171](https://github.com/H3rmt/hyprshell-test/issues/171)) ([d42f12b](https://github.com/H3rmt/hyprshell-test/commit/d42f12be62c08d3764bc91b034bc7aa05d531608))
* rewrite hyprswitch ([198cd0f](https://github.com/H3rmt/hyprshell-test/commit/198cd0f5ae03210b46cfaba8dbf5f8d30fcc77a9))
* rewrite hyprswitch ([0d834ab](https://github.com/H3rmt/hyprshell-test/commit/0d834ab64cb607d2bc10ca7b13d2642728592f50))


### Bug Fixes

* add nix back ([9efadcd](https://github.com/H3rmt/hyprshell-test/commit/9efadcdc37ae0c639ab36a1880e3d48b1b6c51a2))
* allow hold of tab and arrow keys to switch ([e0aaa57](https://github.com/H3rmt/hyprshell-test/commit/e0aaa570f54fce0155631c5720805886a0c275a1))
* close launcher on esc ([a003b82](https://github.com/H3rmt/hyprshell-test/commit/a003b8227b709a7bb186e2b3703ec70f3f16dd7d))
* detect socat path at runtime ([312f667](https://github.com/H3rmt/hyprshell-test/commit/312f6677165a023466a115d8a61f2927b31adc71))
* don't exit launcher when no items ([a4356b7](https://github.com/H3rmt/hyprshell-test/commit/a4356b783c148f61736fcd9e4af23d63b1be7c85))
* filter apps in the launcher by name, exec and details ([79c7076](https://github.com/H3rmt/hyprshell-test/commit/79c7076758a20c81586b9956f1c5fa2fb5cc0094))
* fix ci release ([e498ca1](https://github.com/H3rmt/hyprshell-test/commit/e498ca19f86fcf2d4668fa7582f9320232fd194d))
* fix ci release ([6dc00c7](https://github.com/H3rmt/hyprshell-test/commit/6dc00c719ef7beaea70c86f7851c6de8cdf9117e))
* Fix Nix Build ([02bd2c4](https://github.com/H3rmt/hyprshell-test/commit/02bd2c400616cb96cd21ccc6b2f530143fcb511b))
* fix overflow for selecting workspaces ([9efadcd](https://github.com/H3rmt/hyprshell-test/commit/9efadcdc37ae0c639ab36a1880e3d48b1b6c51a2))
* fix panic when listening for changes on nonexisting file ([2e02509](https://github.com/H3rmt/hyprshell-test/commit/2e025099b4b259844a6f5192a4e9db3db10a00ba))
* fix versions ([899b2cc](https://github.com/H3rmt/hyprshell-test/commit/899b2cc95018a342c2bf45d0fdf0ef971616be7b))
* fix versions ([e9565fd](https://github.com/H3rmt/hyprshell-test/commit/e9565fd627c85ed4fa8fcf26cd35c06ceeaeb2b7))
* fix versions ([8d9a5ee](https://github.com/H3rmt/hyprshell-test/commit/8d9a5eeeee5d74fef9501fec39c5d8692061ec97))
* fix versions ([05f3da6](https://github.com/H3rmt/hyprshell-test/commit/05f3da60197e59d6a882fa0282127ae62091f879))
* fix versions ([b4e0380](https://github.com/H3rmt/hyprshell-test/commit/b4e0380cced9b3ae35e3d8368fb336edc5530274))
* fix versions ([14e4a72](https://github.com/H3rmt/hyprshell-test/commit/14e4a725dfe6b0d70c235db80069238338ec2890))
* fix versions ([89711a6](https://github.com/H3rmt/hyprshell-test/commit/89711a628c45d6286742ac274ad4ec2fe487faed))
* get socat path at buildtime ([a4356b7](https://github.com/H3rmt/hyprshell-test/commit/a4356b783c148f61736fcd9e4af23d63b1be7c85))
* moved size_factor to scale for a more sensible default and bounds check ([a85f3b8](https://github.com/H3rmt/hyprshell-test/commit/a85f3b8948211711acf232b9834f4c9c2afadf61))
* switch window if no results are present in launcher ([50a81a0](https://github.com/H3rmt/hyprshell-test/commit/50a81a0503f8f259374fdafab73e383f930902fe))
* test ([2079219](https://github.com/H3rmt/hyprshell-test/commit/2079219d7e51e827d4c8f544eac4319f4338d163))
* test2 ([134820a](https://github.com/H3rmt/hyprshell-test/commit/134820a039695bbe3b6ebf18669504cd30a40f29))
* test3 ([995909f](https://github.com/H3rmt/hyprshell-test/commit/995909fcf416d233e91554e761ceb21430150b6c))
* test4 ([35e9f5a](https://github.com/H3rmt/hyprshell-test/commit/35e9f5a657b499d6f90265c98804c491b0ab825a))
* try to fix publication to creates.io ([c911a07](https://github.com/H3rmt/hyprshell-test/commit/c911a078e40655fd869df317479a6a93cce508b2))


### Continuous Integration

* fix release-please again ([824bf03](https://github.com/H3rmt/hyprshell-test/commit/824bf032ab3b131121b16d9c16ce9f6a5215c580))
* fix release-please again ([bce2335](https://github.com/H3rmt/hyprshell-test/commit/bce23355cd3f477e95179acadd5d1401544b1822))
* tests ([5b83cdc](https://github.com/H3rmt/hyprshell-test/commit/5b83cdcc2c86cdc68a674d76c537aa673ffbc403))

## [4.0.0](https://github.com/H3rmt/hyprshell-test/compare/4.0.0...4.0.0) (2025-05-06)


### Features

* add calc plugin ([06c8a41](https://github.com/H3rmt/hyprshell-test/commit/06c8a41db42d482b777f158fcf9eb1b23708cc13))
* add run shell commands from launcher ([879cbba](https://github.com/H3rmt/hyprshell-test/commit/879cbba0597281c14b07dadfe03149b4323caf6f))
* add websearch plugin ([1a079d1](https://github.com/H3rmt/hyprshell-test/commit/1a079d1552036f8713ba69f33271475ff4a41103))
* added click on clients and workspaces in overview and switch ([328fc3b](https://github.com/H3rmt/hyprshell-test/commit/328fc3b432b28ad1e390be10f945e1425708d430))
* added systemd generation (use --no-systemd to disable) ([97c2c7f](https://github.com/H3rmt/hyprshell-test/commit/97c2c7f88c3ba221863a43b8adbfb50b444fa841))
* click on entry in launcher works ([87076ef](https://github.com/H3rmt/hyprshell-test/commit/87076ef8c715fc0f7e29a7c2187aa59f17514df5))
* NixOS Support ([#171](https://github.com/H3rmt/hyprshell-test/issues/171)) ([d42f12b](https://github.com/H3rmt/hyprshell-test/commit/d42f12be62c08d3764bc91b034bc7aa05d531608))
* rewrite hyprswitch ([198cd0f](https://github.com/H3rmt/hyprshell-test/commit/198cd0f5ae03210b46cfaba8dbf5f8d30fcc77a9))
* rewrite hyprswitch ([0d834ab](https://github.com/H3rmt/hyprshell-test/commit/0d834ab64cb607d2bc10ca7b13d2642728592f50))


### Bug Fixes

* add nix back ([9efadcd](https://github.com/H3rmt/hyprshell-test/commit/9efadcdc37ae0c639ab36a1880e3d48b1b6c51a2))
* allow hold of tab and arrow keys to switch ([e0aaa57](https://github.com/H3rmt/hyprshell-test/commit/e0aaa570f54fce0155631c5720805886a0c275a1))
* close launcher on esc ([a003b82](https://github.com/H3rmt/hyprshell-test/commit/a003b8227b709a7bb186e2b3703ec70f3f16dd7d))
* detect socat path at runtime ([312f667](https://github.com/H3rmt/hyprshell-test/commit/312f6677165a023466a115d8a61f2927b31adc71))
* don't exit launcher when no items ([a4356b7](https://github.com/H3rmt/hyprshell-test/commit/a4356b783c148f61736fcd9e4af23d63b1be7c85))
* filter apps in the launcher by name, exec and details ([79c7076](https://github.com/H3rmt/hyprshell-test/commit/79c7076758a20c81586b9956f1c5fa2fb5cc0094))
* fix ci release ([e498ca1](https://github.com/H3rmt/hyprshell-test/commit/e498ca19f86fcf2d4668fa7582f9320232fd194d))
* fix ci release ([6dc00c7](https://github.com/H3rmt/hyprshell-test/commit/6dc00c719ef7beaea70c86f7851c6de8cdf9117e))
* Fix Nix Build ([02bd2c4](https://github.com/H3rmt/hyprshell-test/commit/02bd2c400616cb96cd21ccc6b2f530143fcb511b))
* fix overflow for selecting workspaces ([9efadcd](https://github.com/H3rmt/hyprshell-test/commit/9efadcdc37ae0c639ab36a1880e3d48b1b6c51a2))
* fix panic when listening for changes on nonexisting file ([2e02509](https://github.com/H3rmt/hyprshell-test/commit/2e025099b4b259844a6f5192a4e9db3db10a00ba))
* fix versions ([899b2cc](https://github.com/H3rmt/hyprshell-test/commit/899b2cc95018a342c2bf45d0fdf0ef971616be7b))
* fix versions ([e9565fd](https://github.com/H3rmt/hyprshell-test/commit/e9565fd627c85ed4fa8fcf26cd35c06ceeaeb2b7))
* fix versions ([8d9a5ee](https://github.com/H3rmt/hyprshell-test/commit/8d9a5eeeee5d74fef9501fec39c5d8692061ec97))
* fix versions ([05f3da6](https://github.com/H3rmt/hyprshell-test/commit/05f3da60197e59d6a882fa0282127ae62091f879))
* fix versions ([b4e0380](https://github.com/H3rmt/hyprshell-test/commit/b4e0380cced9b3ae35e3d8368fb336edc5530274))
* fix versions ([14e4a72](https://github.com/H3rmt/hyprshell-test/commit/14e4a725dfe6b0d70c235db80069238338ec2890))
* fix versions ([89711a6](https://github.com/H3rmt/hyprshell-test/commit/89711a628c45d6286742ac274ad4ec2fe487faed))
* get socat path at buildtime ([a4356b7](https://github.com/H3rmt/hyprshell-test/commit/a4356b783c148f61736fcd9e4af23d63b1be7c85))
* moved size_factor to scale for a more sensible default and bounds check ([a85f3b8](https://github.com/H3rmt/hyprshell-test/commit/a85f3b8948211711acf232b9834f4c9c2afadf61))
* switch window if no results are present in launcher ([50a81a0](https://github.com/H3rmt/hyprshell-test/commit/50a81a0503f8f259374fdafab73e383f930902fe))
* test ([2079219](https://github.com/H3rmt/hyprshell-test/commit/2079219d7e51e827d4c8f544eac4319f4338d163))
* test2 ([134820a](https://github.com/H3rmt/hyprshell-test/commit/134820a039695bbe3b6ebf18669504cd30a40f29))
* test3 ([995909f](https://github.com/H3rmt/hyprshell-test/commit/995909fcf416d233e91554e761ceb21430150b6c))
* try to fix publication to creates.io ([c911a07](https://github.com/H3rmt/hyprshell-test/commit/c911a078e40655fd869df317479a6a93cce508b2))


### Continuous Integration

* fix release-please again ([824bf03](https://github.com/H3rmt/hyprshell-test/commit/824bf032ab3b131121b16d9c16ce9f6a5215c580))
* fix release-please again ([bce2335](https://github.com/H3rmt/hyprshell-test/commit/bce23355cd3f477e95179acadd5d1401544b1822))
* tests ([5b83cdc](https://github.com/H3rmt/hyprshell-test/commit/5b83cdcc2c86cdc68a674d76c537aa673ffbc403))

## [4.0.0](https://github.com/H3rmt/hyprshell-test/compare/4.0.0...4.0.0) (2025-05-06)


### Features

* add calc plugin ([06c8a41](https://github.com/H3rmt/hyprshell-test/commit/06c8a41db42d482b777f158fcf9eb1b23708cc13))
* add run shell commands from launcher ([879cbba](https://github.com/H3rmt/hyprshell-test/commit/879cbba0597281c14b07dadfe03149b4323caf6f))
* add websearch plugin ([1a079d1](https://github.com/H3rmt/hyprshell-test/commit/1a079d1552036f8713ba69f33271475ff4a41103))
* added click on clients and workspaces in overview and switch ([328fc3b](https://github.com/H3rmt/hyprshell-test/commit/328fc3b432b28ad1e390be10f945e1425708d430))
* added systemd generation (use --no-systemd to disable) ([97c2c7f](https://github.com/H3rmt/hyprshell-test/commit/97c2c7f88c3ba221863a43b8adbfb50b444fa841))
* click on entry in launcher works ([87076ef](https://github.com/H3rmt/hyprshell-test/commit/87076ef8c715fc0f7e29a7c2187aa59f17514df5))
* NixOS Support ([#171](https://github.com/H3rmt/hyprshell-test/issues/171)) ([d42f12b](https://github.com/H3rmt/hyprshell-test/commit/d42f12be62c08d3764bc91b034bc7aa05d531608))
* rewrite hyprswitch ([198cd0f](https://github.com/H3rmt/hyprshell-test/commit/198cd0f5ae03210b46cfaba8dbf5f8d30fcc77a9))
* rewrite hyprswitch ([0d834ab](https://github.com/H3rmt/hyprshell-test/commit/0d834ab64cb607d2bc10ca7b13d2642728592f50))


### Bug Fixes

* add nix back ([9efadcd](https://github.com/H3rmt/hyprshell-test/commit/9efadcdc37ae0c639ab36a1880e3d48b1b6c51a2))
* allow hold of tab and arrow keys to switch ([e0aaa57](https://github.com/H3rmt/hyprshell-test/commit/e0aaa570f54fce0155631c5720805886a0c275a1))
* close launcher on esc ([a003b82](https://github.com/H3rmt/hyprshell-test/commit/a003b8227b709a7bb186e2b3703ec70f3f16dd7d))
* detect socat path at runtime ([312f667](https://github.com/H3rmt/hyprshell-test/commit/312f6677165a023466a115d8a61f2927b31adc71))
* don't exit launcher when no items ([a4356b7](https://github.com/H3rmt/hyprshell-test/commit/a4356b783c148f61736fcd9e4af23d63b1be7c85))
* filter apps in the launcher by name, exec and details ([79c7076](https://github.com/H3rmt/hyprshell-test/commit/79c7076758a20c81586b9956f1c5fa2fb5cc0094))
* fix ci release ([e498ca1](https://github.com/H3rmt/hyprshell-test/commit/e498ca19f86fcf2d4668fa7582f9320232fd194d))
* fix ci release ([6dc00c7](https://github.com/H3rmt/hyprshell-test/commit/6dc00c719ef7beaea70c86f7851c6de8cdf9117e))
* Fix Nix Build ([02bd2c4](https://github.com/H3rmt/hyprshell-test/commit/02bd2c400616cb96cd21ccc6b2f530143fcb511b))
* fix overflow for selecting workspaces ([9efadcd](https://github.com/H3rmt/hyprshell-test/commit/9efadcdc37ae0c639ab36a1880e3d48b1b6c51a2))
* fix panic when listening for changes on nonexisting file ([2e02509](https://github.com/H3rmt/hyprshell-test/commit/2e025099b4b259844a6f5192a4e9db3db10a00ba))
* fix versions ([899b2cc](https://github.com/H3rmt/hyprshell-test/commit/899b2cc95018a342c2bf45d0fdf0ef971616be7b))
* fix versions ([e9565fd](https://github.com/H3rmt/hyprshell-test/commit/e9565fd627c85ed4fa8fcf26cd35c06ceeaeb2b7))
* fix versions ([8d9a5ee](https://github.com/H3rmt/hyprshell-test/commit/8d9a5eeeee5d74fef9501fec39c5d8692061ec97))
* fix versions ([05f3da6](https://github.com/H3rmt/hyprshell-test/commit/05f3da60197e59d6a882fa0282127ae62091f879))
* fix versions ([b4e0380](https://github.com/H3rmt/hyprshell-test/commit/b4e0380cced9b3ae35e3d8368fb336edc5530274))
* fix versions ([14e4a72](https://github.com/H3rmt/hyprshell-test/commit/14e4a725dfe6b0d70c235db80069238338ec2890))
* fix versions ([89711a6](https://github.com/H3rmt/hyprshell-test/commit/89711a628c45d6286742ac274ad4ec2fe487faed))
* get socat path at buildtime ([a4356b7](https://github.com/H3rmt/hyprshell-test/commit/a4356b783c148f61736fcd9e4af23d63b1be7c85))
* moved size_factor to scale for a more sensible default and bounds check ([a85f3b8](https://github.com/H3rmt/hyprshell-test/commit/a85f3b8948211711acf232b9834f4c9c2afadf61))
* switch window if no results are present in launcher ([50a81a0](https://github.com/H3rmt/hyprshell-test/commit/50a81a0503f8f259374fdafab73e383f930902fe))
* test ([2079219](https://github.com/H3rmt/hyprshell-test/commit/2079219d7e51e827d4c8f544eac4319f4338d163))
* test2 ([134820a](https://github.com/H3rmt/hyprshell-test/commit/134820a039695bbe3b6ebf18669504cd30a40f29))
* try to fix publication to creates.io ([c911a07](https://github.com/H3rmt/hyprshell-test/commit/c911a078e40655fd869df317479a6a93cce508b2))


### Continuous Integration

* fix release-please again ([824bf03](https://github.com/H3rmt/hyprshell-test/commit/824bf032ab3b131121b16d9c16ce9f6a5215c580))
* fix release-please again ([bce2335](https://github.com/H3rmt/hyprshell-test/commit/bce23355cd3f477e95179acadd5d1401544b1822))
* tests ([5b83cdc](https://github.com/H3rmt/hyprshell-test/commit/5b83cdcc2c86cdc68a674d76c537aa673ffbc403))

## [4.0.0](https://github.com/H3rmt/hyprshell-test/compare/4.0.0...4.0.0) (2025-05-06)


### Features

* add calc plugin ([06c8a41](https://github.com/H3rmt/hyprshell-test/commit/06c8a41db42d482b777f158fcf9eb1b23708cc13))
* add run shell commands from launcher ([879cbba](https://github.com/H3rmt/hyprshell-test/commit/879cbba0597281c14b07dadfe03149b4323caf6f))
* add websearch plugin ([1a079d1](https://github.com/H3rmt/hyprshell-test/commit/1a079d1552036f8713ba69f33271475ff4a41103))
* added click on clients and workspaces in overview and switch ([328fc3b](https://github.com/H3rmt/hyprshell-test/commit/328fc3b432b28ad1e390be10f945e1425708d430))
* added systemd generation (use --no-systemd to disable) ([97c2c7f](https://github.com/H3rmt/hyprshell-test/commit/97c2c7f88c3ba221863a43b8adbfb50b444fa841))
* click on entry in launcher works ([87076ef](https://github.com/H3rmt/hyprshell-test/commit/87076ef8c715fc0f7e29a7c2187aa59f17514df5))
* NixOS Support ([#171](https://github.com/H3rmt/hyprshell-test/issues/171)) ([d42f12b](https://github.com/H3rmt/hyprshell-test/commit/d42f12be62c08d3764bc91b034bc7aa05d531608))
* rewrite hyprswitch ([198cd0f](https://github.com/H3rmt/hyprshell-test/commit/198cd0f5ae03210b46cfaba8dbf5f8d30fcc77a9))
* rewrite hyprswitch ([0d834ab](https://github.com/H3rmt/hyprshell-test/commit/0d834ab64cb607d2bc10ca7b13d2642728592f50))


### Bug Fixes

* add nix back ([9efadcd](https://github.com/H3rmt/hyprshell-test/commit/9efadcdc37ae0c639ab36a1880e3d48b1b6c51a2))
* allow hold of tab and arrow keys to switch ([e0aaa57](https://github.com/H3rmt/hyprshell-test/commit/e0aaa570f54fce0155631c5720805886a0c275a1))
* close launcher on esc ([a003b82](https://github.com/H3rmt/hyprshell-test/commit/a003b8227b709a7bb186e2b3703ec70f3f16dd7d))
* detect socat path at runtime ([312f667](https://github.com/H3rmt/hyprshell-test/commit/312f6677165a023466a115d8a61f2927b31adc71))
* don't exit launcher when no items ([a4356b7](https://github.com/H3rmt/hyprshell-test/commit/a4356b783c148f61736fcd9e4af23d63b1be7c85))
* filter apps in the launcher by name, exec and details ([79c7076](https://github.com/H3rmt/hyprshell-test/commit/79c7076758a20c81586b9956f1c5fa2fb5cc0094))
* fix ci release ([e498ca1](https://github.com/H3rmt/hyprshell-test/commit/e498ca19f86fcf2d4668fa7582f9320232fd194d))
* fix ci release ([6dc00c7](https://github.com/H3rmt/hyprshell-test/commit/6dc00c719ef7beaea70c86f7851c6de8cdf9117e))
* Fix Nix Build ([02bd2c4](https://github.com/H3rmt/hyprshell-test/commit/02bd2c400616cb96cd21ccc6b2f530143fcb511b))
* fix overflow for selecting workspaces ([9efadcd](https://github.com/H3rmt/hyprshell-test/commit/9efadcdc37ae0c639ab36a1880e3d48b1b6c51a2))
* fix panic when listening for changes on nonexisting file ([2e02509](https://github.com/H3rmt/hyprshell-test/commit/2e025099b4b259844a6f5192a4e9db3db10a00ba))
* fix versions ([899b2cc](https://github.com/H3rmt/hyprshell-test/commit/899b2cc95018a342c2bf45d0fdf0ef971616be7b))
* fix versions ([e9565fd](https://github.com/H3rmt/hyprshell-test/commit/e9565fd627c85ed4fa8fcf26cd35c06ceeaeb2b7))
* fix versions ([8d9a5ee](https://github.com/H3rmt/hyprshell-test/commit/8d9a5eeeee5d74fef9501fec39c5d8692061ec97))
* fix versions ([05f3da6](https://github.com/H3rmt/hyprshell-test/commit/05f3da60197e59d6a882fa0282127ae62091f879))
* fix versions ([b4e0380](https://github.com/H3rmt/hyprshell-test/commit/b4e0380cced9b3ae35e3d8368fb336edc5530274))
* fix versions ([14e4a72](https://github.com/H3rmt/hyprshell-test/commit/14e4a725dfe6b0d70c235db80069238338ec2890))
* fix versions ([89711a6](https://github.com/H3rmt/hyprshell-test/commit/89711a628c45d6286742ac274ad4ec2fe487faed))
* get socat path at buildtime ([a4356b7](https://github.com/H3rmt/hyprshell-test/commit/a4356b783c148f61736fcd9e4af23d63b1be7c85))
* moved size_factor to scale for a more sensible default and bounds check ([a85f3b8](https://github.com/H3rmt/hyprshell-test/commit/a85f3b8948211711acf232b9834f4c9c2afadf61))
* switch window if no results are present in launcher ([50a81a0](https://github.com/H3rmt/hyprshell-test/commit/50a81a0503f8f259374fdafab73e383f930902fe))
* test ([2079219](https://github.com/H3rmt/hyprshell-test/commit/2079219d7e51e827d4c8f544eac4319f4338d163))
* try to fix publication to creates.io ([c911a07](https://github.com/H3rmt/hyprshell-test/commit/c911a078e40655fd869df317479a6a93cce508b2))


### Continuous Integration

* fix release-please again ([824bf03](https://github.com/H3rmt/hyprshell-test/commit/824bf032ab3b131121b16d9c16ce9f6a5215c580))
* fix release-please again ([bce2335](https://github.com/H3rmt/hyprshell-test/commit/bce23355cd3f477e95179acadd5d1401544b1822))
* tests ([5b83cdc](https://github.com/H3rmt/hyprshell-test/commit/5b83cdcc2c86cdc68a674d76c537aa673ffbc403))
