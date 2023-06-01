S=str(input())
T=str(input())
R=[0]*len(S)
flag=True
for i in range(26):
	for j in range(len(S)):
		R[j]=chr((ord(S[j])-97+i)%26+97)
	if R==list(T):
		print('Yes')
		flag=False
		break
if flag:
	print('No')