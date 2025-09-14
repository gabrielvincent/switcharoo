# NixOS

## No flakes

### Without Flakes (nixpkgs-unstable)

`configuration.nix`:

```nix
{pkgs, ...}: {
  environment.systemPackages = [pkgs.hyprshell];
}
```

### Without Flakes with Home-manager

`./user.nix`:

All the settings are optional and can be found in the [config](./CONFIGURE.md)

This config enables overview and switch, but is not typesave like the flake home-manager config.

```nix
{ inputs, ... } : {
  services.hyprshell = {
    enable = true;
    settings = {
      windows = {
        overview = {
        };
        switch = {
        };
      };
    };
  };
}
```

## Flakes

Warning: hyprshell builds a hyprland plugin at runtime which **requires the C headers** from the running hyprland instance.

This is trivial for other platforms, but not for NixOS and can cause problems (please report them on github of you encounter any). The default hyprshell program from nixpkgs has access to the hyprland C headers from nixpkgs-unstable.
The same goes for the default value of the hyprland flake programs output. **If you dont use hyprland from nixpkgs** you must either use the home-manager module and specify the hyprland package or manualy wrap using a helper function.
If you use the home-manager module you can use the `hyprland` option to specify the hyprland package.

The flake exposes a helper function (`wrap-hyprshell`) in the `helpers` output which can be used to wrap the hyprshell program.
It accepts the hyprland package as the first argument and the current nixpkgs as the second argument.

### No Home-manager

**[Cachix Cache](https://app.cachix.org/cache/hyprshell#pull) can be added with `cachix use hyprshell`, please don't override nixpkgs if you use this**

`flake.nix`:

```nix
{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    hyprshell.url = "github:H3rmt/hyprshell";
  };

  outputs = { nixpkgs, hyprshell }: {
    nixosConfigurations.hostname = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      modules = [{ environment.systemPackages = [ 
        nixpkgs.hyprland
        hyprshell.packages.${nixpkgs.stdenv.hostPlatform.system}.hyprshell 
      ]; }];
    };
  };
}
```

### No Home-manager with custom hyprland

**[Cachix Cache](https://app.cachix.org/cache/hyprshell#pull) can be added with `cachix use hyprshell`, please don't override nixpkgs if you use this**

`flake.nix`:

```nix
{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    hyprland.url = "github:hyprwm/Hyprland";
    hyprshell.url = "github:H3rmt/hyprshell";
  };

  outputs = { nixpkgs, hyprland, hyprshell }@inputs: {
    nixosConfigurations.hostname = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      modules = [{ environment.systemPackages = [ 
        hyprland.packages.${nixpkgs.stdenv.hostPlatform.system}.hyprland
        (hyprshell.helpers.wrap-hyprshell hyprland nixpkgs)
      ]; }];
    };
  };
}
```

### With Home-manager [recommend]

**[Cachix Cache](https://app.cachix.org/cache/hyprshell#pull) can be added with `cachix use hyprshell`**

`flake.nix`:

```nix
{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    hyprshell.url = "github:H3rmt/hyprshell";
  };

  outputs = { nixpkgs, hyprshell }@inputs: {
    nixosConfigurations.hostname = nixpkgs.lib.nixosSystem {
      specialArgs = { inherit inputs; };
      system = "x86_64-linux";
      modules = [
        home-manager.nixosModules.home-manager
        ./home.nix
      ];
    };
  };
}
```

`./home.nix`:

```nix
{ inputs, ... } : {
  home-manager = {
    extraSpecialArgs = { inherit inputs; };
    user.test = import ./user.nix; 
  };
}
```

`./user.nix`:

All the settings are optional and can be found in the [module.nix](./nix/module.nix)

Everything is disabled by default, so you need to enable it (even settings.windows if settings.windows.overview is enabled).

```nix
{ inputs, ... } : {
  imports = [
    inputs.hyprshell.homeModules.hyprshell
  ];
  programs.hyprshell = {
    enable = true;
    # use this if you dont use hyprland from nixpkgs
    hyprland = inputs.hyprland.packages.${pkgs.stdenv.hostPlatform.system}.default;
    systemd.args = "-v";
    settings = {
      windows = {
        enable = true; # please dont forget to enable windows if you want to use overview or switch
        overview = {
          enable = true;
          key = "super_l";
          modifier = "super";
          launcher = {
            max_items = 6;
          };
        };
        switch.enable = true;
      };
    };
  };
}
```