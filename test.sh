#!/usr/bin/env bash
set -euo pipefail

export RUSTFLAGS=-Awarnings

# Define the features as an array
declare -a features=("generate_config_command" "json_config" "toml_config" "debug_command" "bar")
#declare -a features=("generate_config_command" "bar")

# Get the total number of features
num_features=${#features[@]}

# Function to build with a specific combination of features
build_with_features() {
  local feature_combination="$1"
  local iteration="$2"

  if [[ -z "$feature_combination" ]]; then
    echo "[$iteration] Building without any features..."
    cargo build --no-default-features --quiet
  else
    echo "[$iteration] Building with features: $feature_combination"
    cargo build --no-default-features --features "$feature_combination" --quiet
  fi
}

# Generate all combinations of features
echo "num_features: $num_features, iterations: $((1 << num_features))"
for ((i = 0; i < (1 << num_features); i++)); do
  combination=()
  for ((j = num_features - 1; j >= 0; j--)); do
    if ((i & (1 << j))); then
      combination+=("${features[j]}")
    fi
  done
  build_with_features "$(IFS=,; printf '%s' "${combination[*]}")" "$i"
done

echo "all features tested"