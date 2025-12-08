s/const //
s/s =>/lambda s: (/
s/;/);/
s/verifyPassword =/verifyPassword = (/
s/||/) | (/g
s/\&\&/\&/g
s/false/myFalse/g
s/true/myTrue/g
s/undefined\|null/None/g
s/\(s\[[^'"]\+["'][^'"]*['"]\)/(\1)/g

