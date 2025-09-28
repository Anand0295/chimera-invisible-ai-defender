#!/usr/bin/env python3
"""
Chimera Invisible AI Defender - Python Components Setup

⚠️ EXPERIMENTAL USE ONLY - LAB ENVIRONMENT RESEARCH PROJECT ⚠️
"""

from setuptools import setup, find_packages

with open("README.md", "r", encoding="utf-8") as fh:
    long_description = fh.read()

with open("requirements.txt", "r", encoding="utf-8") as fh:
    requirements = [line.strip() for line in fh if line.strip() and not line.startswith("#")]

setup(
    name="chimera-ai-defender",
    version="0.1.0",
    author="Chimera Research Team",
    author_email="research@chimera-lab.local",
    description="AI-driven cybersecurity research simulation framework",
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="https://github.com/chimera-lab/invisible-ai-defender",
    packages=find_packages(where="python"),
    package_dir={"": "python"},
    classifiers=[
        "Development Status :: 3 - Alpha",
        "Intended Audience :: Science/Research",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.9",
        "Programming Language :: Python :: 3.10",
        "Programming Language :: Python :: 3.11",
        "Topic :: Security",
        "Topic :: Scientific/Engineering :: Artificial Intelligence",
    ],
    python_requires=">=3.9",
    install_requires=requirements,
    extras_require={
        "dev": [
            "pytest>=7.4.0",
            "pytest-asyncio>=0.21.0", 
            "pytest-cov>=4.1.0",
            "black>=23.7.0",
            "flake8>=6.0.0",
            "mypy>=1.5.0",
        ],
        "docs": [
            "sphinx>=7.1.0",
            "sphinx-rtd-theme>=1.3.0",
        ],
    },
    entry_points={
        "console_scripts": [
            "chimera-ai-firewall=chimera.ai_firewall.main:main",
            "chimera-behavior-monitor=chimera.behavior_monitor.main:main",
            "chimera-ddos-simulator=chimera.ddos_simulator.main:main",
        ],
    },
)