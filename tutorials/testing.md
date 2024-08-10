# Testing

## Setup

In `src/backend/app_test.py` tests have already been setup for the server using the `unittest` module.

They can be run by running the file after installing the requirements. 

The test suit does not require running the server. What it does is:

1. Create a server instance:

```python
    self.app = app.app.test_client()
```

2. Create a test SQLite database:

```python
    self.db = tempfile.NamedTemporaryFile(delete=False)
    app.DATABASE = self.db.name
    app.init_db()
```

3. Run the tests. Here is example of how to hit an endpoint:

```python
self.app.post(
            "/api/register",
            data={
                "username": username,
                "password": password,
                "password2": password2,
                "email": email,
            },
            follow_redirects=True,
        )
```

And an example of an assertion:

```python
        self.assertIn(b"You were successfully registered and can login now", rv.data)
```

4. Close the database and delete the file:

```python
    self.db.close()
    os.unlink(self.db.name)
```

## The test job in the workflow

In `.github/workflows/ci.yml`, the test job checks out the code, sets up python 3 and installs the requirements. The last job runs the tests like we would do locally:

```yaml
      - name: Run tests
        run: |
          python src/backend/app_tests.py
```

