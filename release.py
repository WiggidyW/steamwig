import subprocess
import zipfile
from pathlib import Path
import shutil

def release_archive(name, project_dir):
    out_path = project_dir / "releases" / name
    with zipfile.ZipFile(out_path, 'w', zipfile.ZIP_DEFLATED) as zipf:
        zipf.write(
            project_dir / "target" / "release" / "steamwig.exe",
            arcname = Path("steamwig.exe")
        )
        zipf.write(
            project_dir / "target" / "release" / "assets" / "AudioDeviceCmdlets.dll",
            arcname = Path("assets", "AudioDeviceCmdlets.dll")
        )
        zipf.write(
            project_dir / "target" / "release" / "assets" / "MultiMonitorTool.chm",
            arcname = Path("assets", "MultiMonitorTool.chm")
        )
        zipf.write(
            project_dir / "target" / "release" / "assets" / "MultiMonitorTool.exe",
            arcname = Path("assets", "MultiMonitorTool.exe")
        )
        zipf.write(
            project_dir / "target" / "release" / "assets" / "readme.txt",
            arcname = Path("assets", "readme.txt")
        )

if Path().resolve() != Path(__file__).parent.resolve():
    raise Exception("Script must be run from project root directory")

subprocess.run(
    ["cargo", "build", "-r", "--bin", "steamwig", "--target", "i686-pc-windows-msvc"],
    check = True
)
release_archive("i686-pc-windows-msvc.zip", Path())

subprocess.run(
    ["cargo", "build", "-r", "--bin", "steamwig", "--target", "x86_64-pc-windows-msvc"],
    check = True
)
release_archive("x86_64-pc-windows-msvc.zip", Path())