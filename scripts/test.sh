#!/usr/bin/env bash
set -euo pipefail

export RUSTFLAGS=-Awarnings

# Define the features as an array
#declare -a features=("generate_config_command" "toml_config" "debug_command" "bar" "launcher_calc")
declare -a features=("generate_config_command" "json5_config" "config_check_is_default" "launcher_calc" "debug_command")

# Get the total number of features
num_features=${#features[@]}

# Function to build with a specific combination of features
build_with_features() {
  local feature_combination="$1"
  local iteration="$2"
  local start_time=$(date +%s.%N)

  if [[ -z "$feature_combination" ]]; then
    echo "[$iteration] Running clippy without any features..."
    cargo clippy --no-default-features
  else
    echo "[$iteration] Running clippy with features: $feature_combination"
    cargo clippy --no-default-features --features "$feature_combination"
  fi
  
  local duration=$(awk "BEGIN {print $(date +%s.%N) - $start_time}")
  printf "  took %.2f seconds\n" "$duration"
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