# Flask Variations

This branch focuses on end-to-end testing the Flask application using Playwright. The tests are located in the `src/test/e2e` directory.

## Getting started with Playwright

Follow the tutorial from the documentation:

https://playwright.dev/docs/intro


## Running the tests

First install the dependencies in `src/test/e2e` using `npm install`.


The CLI provides the following suggestions:

```text
 npx playwright test
    Runs the end-to-end tests.

  npx playwright test --ui
    Starts the interactive UI mode.

  npx playwright test --project=chromium
    Runs the tests only on Desktop Chrome.

  npx playwright test example
    Runs the tests in a specific file.

  npx playwright test --debug
    Runs the tests in debug mode.

  npx playwright codegen
    Auto generate tests with Codegen.
```

## Making adjustments for our project

Make sure to run the Flask application. For instance, you could run `make run` in the `src` directory. 

Update the baseURL key in `src/test/e2e/playwright.config.js`:

```js
import { defineConfig } from '@playwright/test';

export default defineConfig({
  testDir: './tests',
  use: {
    baseURL: 'http://localhost:8080',
  },
});
```

A basic test has been created in `basic-functionality.spec.js`. Run it:

```bash
$ npx playwright test
```

You can view a report of the test results:

```bash
$ npx playwright show-report
```

## Github Actions

### Setup

The Playwright CLI setup process offers to create a Github Actions workflow. Note how the workflow [in this repository](.github/workflows/playwright.yml) has been modified, with the `working-directory` key, to reflect the project structure.

It makes sense in this repository to also change the trigger to this:

```yaml
on:
  push:
    branches: end-to-end_testing
  pull_request:
    branches: end-to-end_testing
```

Additional steps have been added to run the application on the runner so that the tests can be run against it.

### Report

You can download the report from the Github Actions workflow. The report is saved as an artifact. Click on download inside of the workflow run.

<img src="./tutorials/github_action_artifacts.png" alt="github actions playwright workflow artifact download report">

The example includes a step to deploy the latest report to GH Pages:

```yaml
    - name: Deploy to GitHub Pages
      if: always()
      uses: peaceiris/actions-gh-pages@v3
      with:
        github_token: ${{ secrets.GITHUB_TOKEN }}
        publish_dir: src/tests/e2e/playwright-report
```

This would make it accessible at `https://<username>.github.io/<repository>/`. 

## Further inspiration

You can record tests by running `npx playwright codegen`. This will generate a test file based on your interactions with the website.

Consider how you will create your test environment. Will you:

- Run the tests in the Github Action runner?
- Run the tests in a test environment before deploying?
- Run the tests after the website has been deployed?

Or maybe a combination of these options?