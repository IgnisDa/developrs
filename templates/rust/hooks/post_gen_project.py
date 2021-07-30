import subprocess

subprocess.check_call(['git', 'init'])
subprocess.check_call(['git', 'add', '.'])
subprocess.check_call(['git', 'commit', '-am', 'feat: initial commit'])
subprocess.check_call(['git', 'checkout', '-b', 'development'])
