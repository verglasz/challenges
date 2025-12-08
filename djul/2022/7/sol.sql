with p as (
	SELECT name, isbn,
       (
         ((isbn / 1) % 10) * 1 +
         ((isbn / 10) % 10) * 3 +
         ((isbn / 100) % 10) * 1 +
         ((isbn / 1000) % 10) * 3 +
         ((isbn / 10000) % 10) * 1 +
         ((isbn / 100000) % 10) * 3 +
         ((isbn / 1000000) % 10) * 1 +
         ((isbn / 10000000) % 10) * 3 +
         ((isbn / 100000000) % 10) * 1 +
         ((isbn / 1000000000) % 10) * 3 +
         ((isbn / 10000000000) % 10) * 1 +
         ((isbn / 100000000000) % 10) * 3 +
         ((isbn / 1000000000000) % 10) * 1
       ) AS weighted_sum from book
) select name from p where weighted_sum % 10 = 0;

