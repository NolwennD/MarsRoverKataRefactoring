import { pathsToModuleNameMapper } from  "ts-jest/utils";
const { compilerOptions } = require("./tsconfig");

export default {
  roots: ["<rootDir>/src"],
  collectCoverage: true,
  coverageDirectory: "coverage",
  coverageProvider: "v8",
  transform: {
    "^.+\\.tsx?$": "ts-jest",
  },
  coverageReporters: [
    "json",
    "text",
    "lcov",
    "clover"
  ],
  moduleNameMapper: pathsToModuleNameMapper(compilerOptions.paths, { prefix: '<rootDir>/' } )
};
