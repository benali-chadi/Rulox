// Generate an AST for rulox (a Rust implementation of the lox language)
import fs from "fs";
import path from "path";

if (process.argv.length !== 3) {
  console.error("Usage: generateAst <outputDir>");
  process.exit(64);
}

const rules = [
  "Binary   : Expr left, Token operator, Expr right",
  "Grouping : Expr expression",
  "Literal  : Object value",
  "Unary    : Token operator, Expr right",
];

const outputDir = process.argv[2];

const srcPath = path.join(__dirname, "src");
