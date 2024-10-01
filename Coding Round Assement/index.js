const fs = require("fs");

// Function to read and parse JSON from a file
function readJsonFile(filename) {
  const data = fs.readFileSync(filename, "utf8");
  return JSON.parse(data);
}

// Function to decode a value given its base
function decodeValue(value, base) {
  return parseInt(value, base);
}

// Function to calculate the secret using Lagrange interpolation
function calculateSecret(points) {
  const secret = points.reduce((result, [xi, yi], i) => {
    let term = yi;
    for (let j = 0; j < points.length; j++) {
      if (i !== j) {
        const [xj] = points[j];
        term *= -xj / (xi - xj);
      }
    }
    return result + term;
  }, 0);

  return Math.round(secret); // Rounding to nearest integer as the secret is expected to be an integer
}

// Main function to process a test case
function processTestCase(filename) {
  const data = readJsonFile(filename);
  const { n, k } = data.keys;

  const points = [];
  for (let i = 1; i <= n && points.length < k; i++) {
    if (data[i]) {
      const x = i;
      const y = decodeValue(data[i].value, parseInt(data[i].base));
      points.push([x, y]);
    }
  }

  if (points.length < k) {
    throw new Error(
      `Not enough points to calculate the secret. Need ${k}, but only have ${points.length}.`
    );
  }

  const secret = calculateSecret(points);
  return secret;
}

// Process both test cases
const secret1 = processTestCase("testcase2.json");
const secret2 = processTestCase("testcase1.json");

console.log("Secret for Test Case 1:", secret1);
console.log("Secret for Test Case 2:", secret2);
