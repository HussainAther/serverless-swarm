import argparse
import os
import yaml
import shutil
from pathlib import Path

TEMPLATE_DIR = Path(__file__).parent.parent / "templates"


def load_spec(spec_path):
    with open(spec_path, 'r') as f:
        return yaml.safe_load(f)


def create_service_dirs(service_name, runtime, output_path):
    service_path = output_path / "lambdas" / service_name
    service_path.mkdir(parents=True, exist_ok=True)

    template_path = TEMPLATE_DIR / runtime.lower()
    if not template_path.exists():
        print(f"[WARN] No templates for runtime: {runtime}. Skipping.")
        return

    for file in template_path.glob("**/*"):
        rel_path = file.relative_to(template_path)
        dest_file = service_path / rel_path
        dest_file.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy(file, dest_file)
        print(f"[INFO] Created: {dest_file}")


def main():
    parser = argparse.ArgumentParser(description="Serverless Swarm Toolkit CLI")
    parser.add_argument("generate", help="Generate a serverless project", nargs='?')
    parser.add_argument("-i", "--input", required=True, help="Path to spec YAML file")
    parser.add_argument("-o", "--output", required=True, help="Output directory")
    args = parser.parse_args()

    spec = load_spec(args.input)
    output_path = Path(args.output)
    output_path.mkdir(parents=True, exist_ok=True)

    services = spec.get("services", {})

    for name, config in services.items():
        runtime = config.get("runtime", "nodejs18.x")
        print(f"[+] Generating service: {name} (runtime: {runtime})")
        create_service_dirs(name, runtime, output_path)

    print("\n✅ Generation complete.")


if __name__ == "__main__":
    main()

