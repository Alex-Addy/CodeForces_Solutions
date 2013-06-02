#include <stdio.h>

int main()
{
	int hr = 0, mi = 0;
	scanf("%d:%d", &hr, &mi);
	printf("%f %d\n", ((hr%12)*30 + mi*0.5)), mi*6;
	return 0;
}