Sentinel
===

![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/abyssnlp/sentinel/ci.yml?label=CI&style=for-the-badge)


![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/abyssnlp/sentinel/release.yml?label=Release&style=for-the-badge)

Dev environment in docker for various OS arch:
```bash
# ubuntu
docker compose -f docker-compose.ubuntu.yml up --build -d
```

To try with current setup:
```bash
target/debug/sentinel run py --name test --path /mnt/f/Python/sentinel/services/test_service.py --pyexec /home/deepabyss
/miniconda3/bin/python
```

TODO
===

- [ ] Use `/var/sentinel` instead of the home dir to save state
- [ ] Tests for services
- [ ] Other commands *
