import "./sub.js";
import file from "spx://resources.spx/asdf.json" assert { type: "json" };

console.log("Hello world!");
console.log("2");

console.log('spx file import: ');
console.log(file);

console.log('dynamic file import: ');
console.log(await import("file:///F:\\StoryWorkspace\\storyboard\\workspace\\asdf.json", { assert: { type: "json" } }));
