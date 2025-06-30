# Whoknows Variations - Security Testing

This tutorial focuses on how to perform various security tests on the WhoKnows project with GitHub Actions.

---

[![Bandit Security Check](https://github.com/who-knows-inc/whoknows_variations/actions/workflows/bandit.yml/badge.svg?branch=security_testing)](https://github.com/who-knows-inc/whoknows_variations/actions/workflows/bandit.yml)
[![OWASP ZAP](https://github.com/who-knows-inc/whoknows_variations/actions/workflows/owasp_zap.yml/badge.svg?branch=security_testing)](https://github.com/who-knows-inc/whoknows_variations/actions/workflows/owasp_zap.yml)
[![Safety Dependency Check](https://github.com/who-knows-inc/whoknows_variations/actions/workflows/safety.yml/badge.svg?branch=security_testing)](https://github.com/who-knows-inc/whoknows_variations/actions/workflows/safety.yml)

**Note**: The entire point is that Bandit workflow fails becauses the codebase has major security issues.

---

## The tutorials

1. [Bandit](./tutorials/01._bandit.md). A static analysis tool for finding security issues in Python code.

2. [Safety](./tutorials/02._safety.md). A CLI tool to check for vulnerable dependencies in your Python projects.

3. [OWASP ZAP](./tutorials/03._owasp_zap.md). A security tool to help automatically find security vulnerabilities in your web applications while you are developing and testing your applications.

