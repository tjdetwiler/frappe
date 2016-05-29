package io.hcf.frappe;

public class Constants {
    public static final int INT_VALUE = 0xcafebabe;
    public static final int INT_MAX = 2147483647;
    public static final int INT_MIN = -2147483648;

    public static final long LONG_VALUE = 0xdeadc0ffeebabeL;
    public static final long LONG_MAX = 9223372036854775807L;
    public static final long LONG_MIN = -9223372036854775808L;

    public static final float FLOAT_MAX = 3.4028235E38F;
    public static final float FLOAT_MIN_NORMAL = 1.17549435E-38F;
    public static final float FLOAT_MIN = 1.4E-45F;
    public static final float FLOAT_NEGATIVE_INF = -1.0F / 0.0F;
    public static final float FLOAT_NAN = 0.0F / 0.0F;
    public static final float FLOAT_POSITIVE_INF = 1.0F / 0.0F;

    public static final double DOUBLE_MAX = 1.7976931348623157E308D;
    public static final double DOUBLE_MIN_NORMAL = 2.2250738585072014E-308D;
    public static final double DOUBLE_MIN = 4.9E-324D;
    public static final double DOUBLE_NEGATIVE_INF = -1.0D / 0.0;
    public static final double DOUBLE_NAN = 0.0D / 0.0;
    public static final double DOUBLE_POSITIVE_INF = 1.0D / 0.0;

    public static final String STRING_VALUE = "This is a string constant";

    public static final Constants CLASS_VALUE = new Constants();
}
