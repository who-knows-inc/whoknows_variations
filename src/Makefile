.PHONY: init run test

init:
	# Indrykning her skal være en TAB, ikke mellemrum!
	PYTHONPATH=backend python3 -c "from app import init_db; init_db()"

run:
	# Indrykning her skal være en TAB
	python3 ./backend/app.py

test:
	# Indrykning her skal være en TAB
	PYTHONPATH=backend python3 ./backend/app_tests.py
