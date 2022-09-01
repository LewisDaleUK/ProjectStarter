# Project Starter

A pretty simple command-line application written in Rust for generating new starter projects. It checks for the following files, in order of precedence:

- ./projects.json
- ~/.config/projects.json
- ~/.projects.json

The `projects.json` files should contain a JSON array of objects:

```json
[
	{
		"title": "Eleventy Starter",
		"language": "Web",
		"description": "Quick starter project for Eleventy with some basic CSS, and predfined templates for blogs and other features",
		"source": "https://github.com/LewisDaleUK/eleventy-starter.git",
		"command": "npm",
		"args": ["install"]
	}
]
```

Once loaded, the application will run you through a wizard that will create a new directory, clone the starter project into that directory, and then run any command arguments.

