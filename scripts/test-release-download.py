#!/usr/bin/env python3
import importlib.util
from pathlib import Path
import tempfile
import unittest

spec = importlib.util.spec_from_file_location('download', Path(__file__).with_name('verify-release-download.py'))
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)


class DownloadTests(unittest.TestCase):
    def test_actual_bytes_names_and_regular_files_are_required(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            expected = root / 'expected'; expected.mkdir()
            downloaded = root / 'downloaded'; downloaded.mkdir()
            with self.assertRaisesRegex(ValueError, 'No release assets'):
                module.verify(expected, downloaded)
            (expected / 'archive').write_bytes(bytes(range(256)))
            with self.assertRaisesRegex(ValueError, 'No release assets'):
                module.verify(expected, downloaded)
            (downloaded / 'archive').write_bytes(bytes(range(256)))
            self.assertEqual(module.verify(expected, downloaded), 1)
            (downloaded / 'archive').write_bytes(bytes(reversed(range(256))))
            with self.assertRaisesRegex(ValueError, 'size/SHA-256'):
                module.verify(expected, downloaded)
            (downloaded / 'archive').write_bytes(bytes(range(256)))
            (downloaded / 'unexpected').write_bytes(b'')
            with self.assertRaisesRegex(ValueError, 'Asset names differ'):
                module.verify(expected, downloaded)
            (downloaded / 'unexpected').unlink()
            (downloaded / 'archive').unlink()
            (downloaded / 'archive').symlink_to(expected / 'archive')
            with self.assertRaisesRegex(ValueError, 'regular assets'):
                module.verify(expected, downloaded)


if __name__ == '__main__':
    unittest.main()
