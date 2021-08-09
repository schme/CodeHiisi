/// Generate glm/glsl style swizzle operators
macro_rules! impl_swizzle_functions {
    ($vector_type1:ident, $vector_type2:ident, $vector_type3:ident, $S:ident, x) => {

        /// Swizzle operator that creates a new type with dimension 1 from variables `x`.
        #[inline] pub fn x(&self) -> $vector_type1<$S> { $vector_type1::new(self.x, ) }

        /// Swizzle operator that creates a new type with dimension 2 from variables `xx`.
        #[inline] pub fn xx(&self) -> $vector_type2<$S> { $vector_type2::new(self.x, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `xxx`.
        #[inline] pub fn xxx(&self) -> $vector_type3<$S> { $vector_type3::new(self.x, self.x, self.x, ) }

    };
    ($vector_type1:ident, $vector_type2:ident, $vector_type3:ident, $S:ident, xy) => {

        /// Swizzle operator that creates a new type with dimension 1 from variables `x`.
        #[inline] pub fn x(&self) -> $vector_type1<$S> { $vector_type1::new(self.x, ) }

        /// Swizzle operator that creates a new type with dimension 1 from variables `y`.
        #[inline] pub fn y(&self) -> $vector_type1<$S> { $vector_type1::new(self.y, ) }

        /// Swizzle operator that creates a new type with dimension 2 from variables `xx`.
        #[inline] pub fn xx(&self) -> $vector_type2<$S> { $vector_type2::new(self.x, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 2 from variables `yx`.
        #[inline] pub fn yx(&self) -> $vector_type2<$S> { $vector_type2::new(self.y, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 2 from variables `xy`.
        #[inline] pub fn xy(&self) -> $vector_type2<$S> { $vector_type2::new(self.x, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 2 from variables `yy`.
        #[inline] pub fn yy(&self) -> $vector_type2<$S> { $vector_type2::new(self.y, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `xxx`.
        #[inline] pub fn xxx(&self) -> $vector_type3<$S> { $vector_type3::new(self.x, self.x, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `yxx`.
        #[inline] pub fn yxx(&self) -> $vector_type3<$S> { $vector_type3::new(self.y, self.x, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `xyx`.
        #[inline] pub fn xyx(&self) -> $vector_type3<$S> { $vector_type3::new(self.x, self.y, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `yyx`.
        #[inline] pub fn yyx(&self) -> $vector_type3<$S> { $vector_type3::new(self.y, self.y, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `xxy`.
        #[inline] pub fn xxy(&self) -> $vector_type3<$S> { $vector_type3::new(self.x, self.x, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `yxy`.
        #[inline] pub fn yxy(&self) -> $vector_type3<$S> { $vector_type3::new(self.y, self.x, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `xyy`.
        #[inline] pub fn xyy(&self) -> $vector_type3<$S> { $vector_type3::new(self.x, self.y, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `yyy`.
        #[inline] pub fn yyy(&self) -> $vector_type3<$S> { $vector_type3::new(self.y, self.y, self.y, ) }

    };
    ($vector_type1:ident, $vector_type2:ident, $vector_type3:ident, $S:ident, xyz) => {

        /// Swizzle operator that creates a new type with dimension 1 from variables `x`.
        #[inline] pub fn x(&self) -> $vector_type1<$S> { $vector_type1::new(self.x, ) }

        /// Swizzle operator that creates a new type with dimension 1 from variables `y`.
        #[inline] pub fn y(&self) -> $vector_type1<$S> { $vector_type1::new(self.y, ) }

        /// Swizzle operator that creates a new type with dimension 1 from variables `z`.
        #[inline] pub fn z(&self) -> $vector_type1<$S> { $vector_type1::new(self.z, ) }

        /// Swizzle operator that creates a new type with dimension 2 from variables `xx`.
        #[inline] pub fn xx(&self) -> $vector_type2<$S> { $vector_type2::new(self.x, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 2 from variables `yx`.
        #[inline] pub fn yx(&self) -> $vector_type2<$S> { $vector_type2::new(self.y, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 2 from variables `zx`.
        #[inline] pub fn zx(&self) -> $vector_type2<$S> { $vector_type2::new(self.z, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 2 from variables `xy`.
        #[inline] pub fn xy(&self) -> $vector_type2<$S> { $vector_type2::new(self.x, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 2 from variables `yy`.
        #[inline] pub fn yy(&self) -> $vector_type2<$S> { $vector_type2::new(self.y, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 2 from variables `zy`.
        #[inline] pub fn zy(&self) -> $vector_type2<$S> { $vector_type2::new(self.z, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 2 from variables `xz`.
        #[inline] pub fn xz(&self) -> $vector_type2<$S> { $vector_type2::new(self.x, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 2 from variables `yz`.
        #[inline] pub fn yz(&self) -> $vector_type2<$S> { $vector_type2::new(self.y, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 2 from variables `zz`.
        #[inline] pub fn zz(&self) -> $vector_type2<$S> { $vector_type2::new(self.z, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `xxx`.
        #[inline] pub fn xxx(&self) -> $vector_type3<$S> { $vector_type3::new(self.x, self.x, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `yxx`.
        #[inline] pub fn yxx(&self) -> $vector_type3<$S> { $vector_type3::new(self.y, self.x, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `zxx`.
        #[inline] pub fn zxx(&self) -> $vector_type3<$S> { $vector_type3::new(self.z, self.x, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `xyx`.
        #[inline] pub fn xyx(&self) -> $vector_type3<$S> { $vector_type3::new(self.x, self.y, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `yyx`.
        #[inline] pub fn yyx(&self) -> $vector_type3<$S> { $vector_type3::new(self.y, self.y, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `zyx`.
        #[inline] pub fn zyx(&self) -> $vector_type3<$S> { $vector_type3::new(self.z, self.y, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `xzx`.
        #[inline] pub fn xzx(&self) -> $vector_type3<$S> { $vector_type3::new(self.x, self.z, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `yzx`.
        #[inline] pub fn yzx(&self) -> $vector_type3<$S> { $vector_type3::new(self.y, self.z, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `zzx`.
        #[inline] pub fn zzx(&self) -> $vector_type3<$S> { $vector_type3::new(self.z, self.z, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `xxy`.
        #[inline] pub fn xxy(&self) -> $vector_type3<$S> { $vector_type3::new(self.x, self.x, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `yxy`.
        #[inline] pub fn yxy(&self) -> $vector_type3<$S> { $vector_type3::new(self.y, self.x, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `zxy`.
        #[inline] pub fn zxy(&self) -> $vector_type3<$S> { $vector_type3::new(self.z, self.x, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `xyy`.
        #[inline] pub fn xyy(&self) -> $vector_type3<$S> { $vector_type3::new(self.x, self.y, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `yyy`.
        #[inline] pub fn yyy(&self) -> $vector_type3<$S> { $vector_type3::new(self.y, self.y, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `zyy`.
        #[inline] pub fn zyy(&self) -> $vector_type3<$S> { $vector_type3::new(self.z, self.y, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `xzy`.
        #[inline] pub fn xzy(&self) -> $vector_type3<$S> { $vector_type3::new(self.x, self.z, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `yzy`.
        #[inline] pub fn yzy(&self) -> $vector_type3<$S> { $vector_type3::new(self.y, self.z, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `zzy`.
        #[inline] pub fn zzy(&self) -> $vector_type3<$S> { $vector_type3::new(self.z, self.z, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `xxz`.
        #[inline] pub fn xxz(&self) -> $vector_type3<$S> { $vector_type3::new(self.x, self.x, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `yxz`.
        #[inline] pub fn yxz(&self) -> $vector_type3<$S> { $vector_type3::new(self.y, self.x, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `zxz`.
        #[inline] pub fn zxz(&self) -> $vector_type3<$S> { $vector_type3::new(self.z, self.x, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `xyz`.
        #[inline] pub fn xyz(&self) -> $vector_type3<$S> { $vector_type3::new(self.x, self.y, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `yyz`.
        #[inline] pub fn yyz(&self) -> $vector_type3<$S> { $vector_type3::new(self.y, self.y, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `zyz`.
        #[inline] pub fn zyz(&self) -> $vector_type3<$S> { $vector_type3::new(self.z, self.y, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `xzz`.
        #[inline] pub fn xzz(&self) -> $vector_type3<$S> { $vector_type3::new(self.x, self.z, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `yzz`.
        #[inline] pub fn yzz(&self) -> $vector_type3<$S> { $vector_type3::new(self.y, self.z, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `zzz`.
        #[inline] pub fn zzz(&self) -> $vector_type3<$S> { $vector_type3::new(self.z, self.z, self.z, ) }

    };
    ($vector_type1:ident, $vector_type2:ident, $vector_type3:ident, $vector_type4:ident, $S:ident, x) => {

        /// Swizzle operator that creates a new type with dimension 1 from variables `x`.
        #[inline] pub fn x(&self) -> $vector_type1<$S> { $vector_type1::new(self.x, ) }

        /// Swizzle operator that creates a new type with dimension 2 from variables `xx`.
        #[inline] pub fn xx(&self) -> $vector_type2<$S> { $vector_type2::new(self.x, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `xxx`.
        #[inline] pub fn xxx(&self) -> $vector_type3<$S> { $vector_type3::new(self.x, self.x, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xxxx`.
        #[inline] pub fn xxxx(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.x, self.x, self.x, ) }

    };
    ($vector_type1:ident, $vector_type2:ident, $vector_type3:ident, $vector_type4:ident, $S:ident, xy) => {

        /// Swizzle operator that creates a new type with dimension 1 from variables `x`.
        #[inline] pub fn x(&self) -> $vector_type1<$S> { $vector_type1::new(self.x, ) }

        /// Swizzle operator that creates a new type with dimension 1 from variables `y`.
        #[inline] pub fn y(&self) -> $vector_type1<$S> { $vector_type1::new(self.y, ) }

        /// Swizzle operator that creates a new type with dimension 2 from variables `xx`.
        #[inline] pub fn xx(&self) -> $vector_type2<$S> { $vector_type2::new(self.x, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 2 from variables `yx`.
        #[inline] pub fn yx(&self) -> $vector_type2<$S> { $vector_type2::new(self.y, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 2 from variables `xy`.
        #[inline] pub fn xy(&self) -> $vector_type2<$S> { $vector_type2::new(self.x, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 2 from variables `yy`.
        #[inline] pub fn yy(&self) -> $vector_type2<$S> { $vector_type2::new(self.y, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `xxx`.
        #[inline] pub fn xxx(&self) -> $vector_type3<$S> { $vector_type3::new(self.x, self.x, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `yxx`.
        #[inline] pub fn yxx(&self) -> $vector_type3<$S> { $vector_type3::new(self.y, self.x, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `xyx`.
        #[inline] pub fn xyx(&self) -> $vector_type3<$S> { $vector_type3::new(self.x, self.y, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `yyx`.
        #[inline] pub fn yyx(&self) -> $vector_type3<$S> { $vector_type3::new(self.y, self.y, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `xxy`.
        #[inline] pub fn xxy(&self) -> $vector_type3<$S> { $vector_type3::new(self.x, self.x, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `yxy`.
        #[inline] pub fn yxy(&self) -> $vector_type3<$S> { $vector_type3::new(self.y, self.x, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `xyy`.
        #[inline] pub fn xyy(&self) -> $vector_type3<$S> { $vector_type3::new(self.x, self.y, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `yyy`.
        #[inline] pub fn yyy(&self) -> $vector_type3<$S> { $vector_type3::new(self.y, self.y, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xxxx`.
        #[inline] pub fn xxxx(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.x, self.x, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yxxx`.
        #[inline] pub fn yxxx(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.x, self.x, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xyxx`.
        #[inline] pub fn xyxx(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.y, self.x, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yyxx`.
        #[inline] pub fn yyxx(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.y, self.x, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xxyx`.
        #[inline] pub fn xxyx(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.x, self.y, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yxyx`.
        #[inline] pub fn yxyx(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.x, self.y, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xyyx`.
        #[inline] pub fn xyyx(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.y, self.y, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yyyx`.
        #[inline] pub fn yyyx(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.y, self.y, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xxxy`.
        #[inline] pub fn xxxy(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.x, self.x, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yxxy`.
        #[inline] pub fn yxxy(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.x, self.x, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xyxy`.
        #[inline] pub fn xyxy(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.y, self.x, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yyxy`.
        #[inline] pub fn yyxy(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.y, self.x, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xxyy`.
        #[inline] pub fn xxyy(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.x, self.y, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yxyy`.
        #[inline] pub fn yxyy(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.x, self.y, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xyyy`.
        #[inline] pub fn xyyy(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.y, self.y, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yyyy`.
        #[inline] pub fn yyyy(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.y, self.y, self.y, ) }

    };
    ($vector_type1:ident, $vector_type2:ident, $vector_type3:ident, $vector_type4:ident, $S:ident, xyz) => {

        /// Swizzle operator that creates a new type with dimension 1 from variables `x`.
        #[inline] pub fn x(&self) -> $vector_type1<$S> { $vector_type1::new(self.x, ) }

        /// Swizzle operator that creates a new type with dimension 1 from variables `y`.
        #[inline] pub fn y(&self) -> $vector_type1<$S> { $vector_type1::new(self.y, ) }

        /// Swizzle operator that creates a new type with dimension 1 from variables `z`.
        #[inline] pub fn z(&self) -> $vector_type1<$S> { $vector_type1::new(self.z, ) }

        /// Swizzle operator that creates a new type with dimension 2 from variables `xx`.
        #[inline] pub fn xx(&self) -> $vector_type2<$S> { $vector_type2::new(self.x, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 2 from variables `yx`.
        #[inline] pub fn yx(&self) -> $vector_type2<$S> { $vector_type2::new(self.y, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 2 from variables `zx`.
        #[inline] pub fn zx(&self) -> $vector_type2<$S> { $vector_type2::new(self.z, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 2 from variables `xy`.
        #[inline] pub fn xy(&self) -> $vector_type2<$S> { $vector_type2::new(self.x, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 2 from variables `yy`.
        #[inline] pub fn yy(&self) -> $vector_type2<$S> { $vector_type2::new(self.y, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 2 from variables `zy`.
        #[inline] pub fn zy(&self) -> $vector_type2<$S> { $vector_type2::new(self.z, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 2 from variables `xz`.
        #[inline] pub fn xz(&self) -> $vector_type2<$S> { $vector_type2::new(self.x, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 2 from variables `yz`.
        #[inline] pub fn yz(&self) -> $vector_type2<$S> { $vector_type2::new(self.y, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 2 from variables `zz`.
        #[inline] pub fn zz(&self) -> $vector_type2<$S> { $vector_type2::new(self.z, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `xxx`.
        #[inline] pub fn xxx(&self) -> $vector_type3<$S> { $vector_type3::new(self.x, self.x, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `yxx`.
        #[inline] pub fn yxx(&self) -> $vector_type3<$S> { $vector_type3::new(self.y, self.x, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `zxx`.
        #[inline] pub fn zxx(&self) -> $vector_type3<$S> { $vector_type3::new(self.z, self.x, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `xyx`.
        #[inline] pub fn xyx(&self) -> $vector_type3<$S> { $vector_type3::new(self.x, self.y, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `yyx`.
        #[inline] pub fn yyx(&self) -> $vector_type3<$S> { $vector_type3::new(self.y, self.y, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `zyx`.
        #[inline] pub fn zyx(&self) -> $vector_type3<$S> { $vector_type3::new(self.z, self.y, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `xzx`.
        #[inline] pub fn xzx(&self) -> $vector_type3<$S> { $vector_type3::new(self.x, self.z, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `yzx`.
        #[inline] pub fn yzx(&self) -> $vector_type3<$S> { $vector_type3::new(self.y, self.z, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `zzx`.
        #[inline] pub fn zzx(&self) -> $vector_type3<$S> { $vector_type3::new(self.z, self.z, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `xxy`.
        #[inline] pub fn xxy(&self) -> $vector_type3<$S> { $vector_type3::new(self.x, self.x, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `yxy`.
        #[inline] pub fn yxy(&self) -> $vector_type3<$S> { $vector_type3::new(self.y, self.x, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `zxy`.
        #[inline] pub fn zxy(&self) -> $vector_type3<$S> { $vector_type3::new(self.z, self.x, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `xyy`.
        #[inline] pub fn xyy(&self) -> $vector_type3<$S> { $vector_type3::new(self.x, self.y, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `yyy`.
        #[inline] pub fn yyy(&self) -> $vector_type3<$S> { $vector_type3::new(self.y, self.y, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `zyy`.
        #[inline] pub fn zyy(&self) -> $vector_type3<$S> { $vector_type3::new(self.z, self.y, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `xzy`.
        #[inline] pub fn xzy(&self) -> $vector_type3<$S> { $vector_type3::new(self.x, self.z, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `yzy`.
        #[inline] pub fn yzy(&self) -> $vector_type3<$S> { $vector_type3::new(self.y, self.z, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `zzy`.
        #[inline] pub fn zzy(&self) -> $vector_type3<$S> { $vector_type3::new(self.z, self.z, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `xxz`.
        #[inline] pub fn xxz(&self) -> $vector_type3<$S> { $vector_type3::new(self.x, self.x, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `yxz`.
        #[inline] pub fn yxz(&self) -> $vector_type3<$S> { $vector_type3::new(self.y, self.x, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `zxz`.
        #[inline] pub fn zxz(&self) -> $vector_type3<$S> { $vector_type3::new(self.z, self.x, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `xyz`.
        #[inline] pub fn xyz(&self) -> $vector_type3<$S> { $vector_type3::new(self.x, self.y, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `yyz`.
        #[inline] pub fn yyz(&self) -> $vector_type3<$S> { $vector_type3::new(self.y, self.y, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `zyz`.
        #[inline] pub fn zyz(&self) -> $vector_type3<$S> { $vector_type3::new(self.z, self.y, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `xzz`.
        #[inline] pub fn xzz(&self) -> $vector_type3<$S> { $vector_type3::new(self.x, self.z, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `yzz`.
        #[inline] pub fn yzz(&self) -> $vector_type3<$S> { $vector_type3::new(self.y, self.z, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `zzz`.
        #[inline] pub fn zzz(&self) -> $vector_type3<$S> { $vector_type3::new(self.z, self.z, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xxxx`.
        #[inline] pub fn xxxx(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.x, self.x, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yxxx`.
        #[inline] pub fn yxxx(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.x, self.x, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zxxx`.
        #[inline] pub fn zxxx(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.x, self.x, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xyxx`.
        #[inline] pub fn xyxx(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.y, self.x, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yyxx`.
        #[inline] pub fn yyxx(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.y, self.x, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zyxx`.
        #[inline] pub fn zyxx(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.y, self.x, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xzxx`.
        #[inline] pub fn xzxx(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.z, self.x, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yzxx`.
        #[inline] pub fn yzxx(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.z, self.x, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zzxx`.
        #[inline] pub fn zzxx(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.z, self.x, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xxyx`.
        #[inline] pub fn xxyx(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.x, self.y, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yxyx`.
        #[inline] pub fn yxyx(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.x, self.y, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zxyx`.
        #[inline] pub fn zxyx(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.x, self.y, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xyyx`.
        #[inline] pub fn xyyx(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.y, self.y, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yyyx`.
        #[inline] pub fn yyyx(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.y, self.y, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zyyx`.
        #[inline] pub fn zyyx(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.y, self.y, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xzyx`.
        #[inline] pub fn xzyx(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.z, self.y, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yzyx`.
        #[inline] pub fn yzyx(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.z, self.y, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zzyx`.
        #[inline] pub fn zzyx(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.z, self.y, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xxzx`.
        #[inline] pub fn xxzx(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.x, self.z, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yxzx`.
        #[inline] pub fn yxzx(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.x, self.z, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zxzx`.
        #[inline] pub fn zxzx(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.x, self.z, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xyzx`.
        #[inline] pub fn xyzx(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.y, self.z, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yyzx`.
        #[inline] pub fn yyzx(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.y, self.z, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zyzx`.
        #[inline] pub fn zyzx(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.y, self.z, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xzzx`.
        #[inline] pub fn xzzx(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.z, self.z, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yzzx`.
        #[inline] pub fn yzzx(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.z, self.z, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zzzx`.
        #[inline] pub fn zzzx(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.z, self.z, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xxxy`.
        #[inline] pub fn xxxy(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.x, self.x, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yxxy`.
        #[inline] pub fn yxxy(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.x, self.x, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zxxy`.
        #[inline] pub fn zxxy(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.x, self.x, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xyxy`.
        #[inline] pub fn xyxy(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.y, self.x, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yyxy`.
        #[inline] pub fn yyxy(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.y, self.x, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zyxy`.
        #[inline] pub fn zyxy(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.y, self.x, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xzxy`.
        #[inline] pub fn xzxy(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.z, self.x, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yzxy`.
        #[inline] pub fn yzxy(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.z, self.x, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zzxy`.
        #[inline] pub fn zzxy(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.z, self.x, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xxyy`.
        #[inline] pub fn xxyy(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.x, self.y, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yxyy`.
        #[inline] pub fn yxyy(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.x, self.y, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zxyy`.
        #[inline] pub fn zxyy(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.x, self.y, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xyyy`.
        #[inline] pub fn xyyy(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.y, self.y, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yyyy`.
        #[inline] pub fn yyyy(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.y, self.y, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zyyy`.
        #[inline] pub fn zyyy(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.y, self.y, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xzyy`.
        #[inline] pub fn xzyy(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.z, self.y, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yzyy`.
        #[inline] pub fn yzyy(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.z, self.y, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zzyy`.
        #[inline] pub fn zzyy(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.z, self.y, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xxzy`.
        #[inline] pub fn xxzy(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.x, self.z, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yxzy`.
        #[inline] pub fn yxzy(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.x, self.z, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zxzy`.
        #[inline] pub fn zxzy(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.x, self.z, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xyzy`.
        #[inline] pub fn xyzy(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.y, self.z, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yyzy`.
        #[inline] pub fn yyzy(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.y, self.z, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zyzy`.
        #[inline] pub fn zyzy(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.y, self.z, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xzzy`.
        #[inline] pub fn xzzy(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.z, self.z, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yzzy`.
        #[inline] pub fn yzzy(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.z, self.z, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zzzy`.
        #[inline] pub fn zzzy(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.z, self.z, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xxxz`.
        #[inline] pub fn xxxz(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.x, self.x, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yxxz`.
        #[inline] pub fn yxxz(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.x, self.x, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zxxz`.
        #[inline] pub fn zxxz(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.x, self.x, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xyxz`.
        #[inline] pub fn xyxz(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.y, self.x, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yyxz`.
        #[inline] pub fn yyxz(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.y, self.x, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zyxz`.
        #[inline] pub fn zyxz(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.y, self.x, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xzxz`.
        #[inline] pub fn xzxz(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.z, self.x, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yzxz`.
        #[inline] pub fn yzxz(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.z, self.x, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zzxz`.
        #[inline] pub fn zzxz(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.z, self.x, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xxyz`.
        #[inline] pub fn xxyz(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.x, self.y, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yxyz`.
        #[inline] pub fn yxyz(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.x, self.y, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zxyz`.
        #[inline] pub fn zxyz(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.x, self.y, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xyyz`.
        #[inline] pub fn xyyz(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.y, self.y, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yyyz`.
        #[inline] pub fn yyyz(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.y, self.y, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zyyz`.
        #[inline] pub fn zyyz(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.y, self.y, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xzyz`.
        #[inline] pub fn xzyz(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.z, self.y, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yzyz`.
        #[inline] pub fn yzyz(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.z, self.y, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zzyz`.
        #[inline] pub fn zzyz(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.z, self.y, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xxzz`.
        #[inline] pub fn xxzz(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.x, self.z, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yxzz`.
        #[inline] pub fn yxzz(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.x, self.z, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zxzz`.
        #[inline] pub fn zxzz(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.x, self.z, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xyzz`.
        #[inline] pub fn xyzz(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.y, self.z, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yyzz`.
        #[inline] pub fn yyzz(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.y, self.z, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zyzz`.
        #[inline] pub fn zyzz(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.y, self.z, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xzzz`.
        #[inline] pub fn xzzz(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.z, self.z, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yzzz`.
        #[inline] pub fn yzzz(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.z, self.z, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zzzz`.
        #[inline] pub fn zzzz(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.z, self.z, self.z, ) }

    };
    ($vector_type1:ident, $vector_type2:ident, $vector_type3:ident, $vector_type4:ident, $S:ident, xyzw) => {

        /// Swizzle operator that creates a new type with dimension 1 from variables `x`.
        #[inline] pub fn x(&self) -> $vector_type1<$S> { $vector_type1::new(self.x, ) }

        /// Swizzle operator that creates a new type with dimension 1 from variables `y`.
        #[inline] pub fn y(&self) -> $vector_type1<$S> { $vector_type1::new(self.y, ) }

        /// Swizzle operator that creates a new type with dimension 1 from variables `z`.
        #[inline] pub fn z(&self) -> $vector_type1<$S> { $vector_type1::new(self.z, ) }

        /// Swizzle operator that creates a new type with dimension 1 from variables `w`.
        #[inline] pub fn w(&self) -> $vector_type1<$S> { $vector_type1::new(self.w, ) }

        /// Swizzle operator that creates a new type with dimension 2 from variables `xx`.
        #[inline] pub fn xx(&self) -> $vector_type2<$S> { $vector_type2::new(self.x, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 2 from variables `yx`.
        #[inline] pub fn yx(&self) -> $vector_type2<$S> { $vector_type2::new(self.y, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 2 from variables `zx`.
        #[inline] pub fn zx(&self) -> $vector_type2<$S> { $vector_type2::new(self.z, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 2 from variables `wx`.
        #[inline] pub fn wx(&self) -> $vector_type2<$S> { $vector_type2::new(self.w, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 2 from variables `xy`.
        #[inline] pub fn xy(&self) -> $vector_type2<$S> { $vector_type2::new(self.x, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 2 from variables `yy`.
        #[inline] pub fn yy(&self) -> $vector_type2<$S> { $vector_type2::new(self.y, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 2 from variables `zy`.
        #[inline] pub fn zy(&self) -> $vector_type2<$S> { $vector_type2::new(self.z, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 2 from variables `wy`.
        #[inline] pub fn wy(&self) -> $vector_type2<$S> { $vector_type2::new(self.w, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 2 from variables `xz`.
        #[inline] pub fn xz(&self) -> $vector_type2<$S> { $vector_type2::new(self.x, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 2 from variables `yz`.
        #[inline] pub fn yz(&self) -> $vector_type2<$S> { $vector_type2::new(self.y, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 2 from variables `zz`.
        #[inline] pub fn zz(&self) -> $vector_type2<$S> { $vector_type2::new(self.z, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 2 from variables `wz`.
        #[inline] pub fn wz(&self) -> $vector_type2<$S> { $vector_type2::new(self.w, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 2 from variables `xw`.
        #[inline] pub fn xw(&self) -> $vector_type2<$S> { $vector_type2::new(self.x, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 2 from variables `yw`.
        #[inline] pub fn yw(&self) -> $vector_type2<$S> { $vector_type2::new(self.y, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 2 from variables `zw`.
        #[inline] pub fn zw(&self) -> $vector_type2<$S> { $vector_type2::new(self.z, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 2 from variables `ww`.
        #[inline] pub fn ww(&self) -> $vector_type2<$S> { $vector_type2::new(self.w, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `xxx`.
        #[inline] pub fn xxx(&self) -> $vector_type3<$S> { $vector_type3::new(self.x, self.x, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `yxx`.
        #[inline] pub fn yxx(&self) -> $vector_type3<$S> { $vector_type3::new(self.y, self.x, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `zxx`.
        #[inline] pub fn zxx(&self) -> $vector_type3<$S> { $vector_type3::new(self.z, self.x, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `wxx`.
        #[inline] pub fn wxx(&self) -> $vector_type3<$S> { $vector_type3::new(self.w, self.x, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `xyx`.
        #[inline] pub fn xyx(&self) -> $vector_type3<$S> { $vector_type3::new(self.x, self.y, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `yyx`.
        #[inline] pub fn yyx(&self) -> $vector_type3<$S> { $vector_type3::new(self.y, self.y, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `zyx`.
        #[inline] pub fn zyx(&self) -> $vector_type3<$S> { $vector_type3::new(self.z, self.y, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `wyx`.
        #[inline] pub fn wyx(&self) -> $vector_type3<$S> { $vector_type3::new(self.w, self.y, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `xzx`.
        #[inline] pub fn xzx(&self) -> $vector_type3<$S> { $vector_type3::new(self.x, self.z, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `yzx`.
        #[inline] pub fn yzx(&self) -> $vector_type3<$S> { $vector_type3::new(self.y, self.z, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `zzx`.
        #[inline] pub fn zzx(&self) -> $vector_type3<$S> { $vector_type3::new(self.z, self.z, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `wzx`.
        #[inline] pub fn wzx(&self) -> $vector_type3<$S> { $vector_type3::new(self.w, self.z, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `xwx`.
        #[inline] pub fn xwx(&self) -> $vector_type3<$S> { $vector_type3::new(self.x, self.w, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `ywx`.
        #[inline] pub fn ywx(&self) -> $vector_type3<$S> { $vector_type3::new(self.y, self.w, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `zwx`.
        #[inline] pub fn zwx(&self) -> $vector_type3<$S> { $vector_type3::new(self.z, self.w, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `wwx`.
        #[inline] pub fn wwx(&self) -> $vector_type3<$S> { $vector_type3::new(self.w, self.w, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `xxy`.
        #[inline] pub fn xxy(&self) -> $vector_type3<$S> { $vector_type3::new(self.x, self.x, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `yxy`.
        #[inline] pub fn yxy(&self) -> $vector_type3<$S> { $vector_type3::new(self.y, self.x, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `zxy`.
        #[inline] pub fn zxy(&self) -> $vector_type3<$S> { $vector_type3::new(self.z, self.x, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `wxy`.
        #[inline] pub fn wxy(&self) -> $vector_type3<$S> { $vector_type3::new(self.w, self.x, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `xyy`.
        #[inline] pub fn xyy(&self) -> $vector_type3<$S> { $vector_type3::new(self.x, self.y, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `yyy`.
        #[inline] pub fn yyy(&self) -> $vector_type3<$S> { $vector_type3::new(self.y, self.y, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `zyy`.
        #[inline] pub fn zyy(&self) -> $vector_type3<$S> { $vector_type3::new(self.z, self.y, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `wyy`.
        #[inline] pub fn wyy(&self) -> $vector_type3<$S> { $vector_type3::new(self.w, self.y, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `xzy`.
        #[inline] pub fn xzy(&self) -> $vector_type3<$S> { $vector_type3::new(self.x, self.z, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `yzy`.
        #[inline] pub fn yzy(&self) -> $vector_type3<$S> { $vector_type3::new(self.y, self.z, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `zzy`.
        #[inline] pub fn zzy(&self) -> $vector_type3<$S> { $vector_type3::new(self.z, self.z, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `wzy`.
        #[inline] pub fn wzy(&self) -> $vector_type3<$S> { $vector_type3::new(self.w, self.z, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `xwy`.
        #[inline] pub fn xwy(&self) -> $vector_type3<$S> { $vector_type3::new(self.x, self.w, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `ywy`.
        #[inline] pub fn ywy(&self) -> $vector_type3<$S> { $vector_type3::new(self.y, self.w, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `zwy`.
        #[inline] pub fn zwy(&self) -> $vector_type3<$S> { $vector_type3::new(self.z, self.w, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `wwy`.
        #[inline] pub fn wwy(&self) -> $vector_type3<$S> { $vector_type3::new(self.w, self.w, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `xxz`.
        #[inline] pub fn xxz(&self) -> $vector_type3<$S> { $vector_type3::new(self.x, self.x, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `yxz`.
        #[inline] pub fn yxz(&self) -> $vector_type3<$S> { $vector_type3::new(self.y, self.x, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `zxz`.
        #[inline] pub fn zxz(&self) -> $vector_type3<$S> { $vector_type3::new(self.z, self.x, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `wxz`.
        #[inline] pub fn wxz(&self) -> $vector_type3<$S> { $vector_type3::new(self.w, self.x, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `xyz`.
        #[inline] pub fn xyz(&self) -> $vector_type3<$S> { $vector_type3::new(self.x, self.y, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `yyz`.
        #[inline] pub fn yyz(&self) -> $vector_type3<$S> { $vector_type3::new(self.y, self.y, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `zyz`.
        #[inline] pub fn zyz(&self) -> $vector_type3<$S> { $vector_type3::new(self.z, self.y, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `wyz`.
        #[inline] pub fn wyz(&self) -> $vector_type3<$S> { $vector_type3::new(self.w, self.y, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `xzz`.
        #[inline] pub fn xzz(&self) -> $vector_type3<$S> { $vector_type3::new(self.x, self.z, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `yzz`.
        #[inline] pub fn yzz(&self) -> $vector_type3<$S> { $vector_type3::new(self.y, self.z, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `zzz`.
        #[inline] pub fn zzz(&self) -> $vector_type3<$S> { $vector_type3::new(self.z, self.z, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `wzz`.
        #[inline] pub fn wzz(&self) -> $vector_type3<$S> { $vector_type3::new(self.w, self.z, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `xwz`.
        #[inline] pub fn xwz(&self) -> $vector_type3<$S> { $vector_type3::new(self.x, self.w, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `ywz`.
        #[inline] pub fn ywz(&self) -> $vector_type3<$S> { $vector_type3::new(self.y, self.w, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `zwz`.
        #[inline] pub fn zwz(&self) -> $vector_type3<$S> { $vector_type3::new(self.z, self.w, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `wwz`.
        #[inline] pub fn wwz(&self) -> $vector_type3<$S> { $vector_type3::new(self.w, self.w, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `xxw`.
        #[inline] pub fn xxw(&self) -> $vector_type3<$S> { $vector_type3::new(self.x, self.x, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `yxw`.
        #[inline] pub fn yxw(&self) -> $vector_type3<$S> { $vector_type3::new(self.y, self.x, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `zxw`.
        #[inline] pub fn zxw(&self) -> $vector_type3<$S> { $vector_type3::new(self.z, self.x, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `wxw`.
        #[inline] pub fn wxw(&self) -> $vector_type3<$S> { $vector_type3::new(self.w, self.x, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `xyw`.
        #[inline] pub fn xyw(&self) -> $vector_type3<$S> { $vector_type3::new(self.x, self.y, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `yyw`.
        #[inline] pub fn yyw(&self) -> $vector_type3<$S> { $vector_type3::new(self.y, self.y, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `zyw`.
        #[inline] pub fn zyw(&self) -> $vector_type3<$S> { $vector_type3::new(self.z, self.y, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `wyw`.
        #[inline] pub fn wyw(&self) -> $vector_type3<$S> { $vector_type3::new(self.w, self.y, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `xzw`.
        #[inline] pub fn xzw(&self) -> $vector_type3<$S> { $vector_type3::new(self.x, self.z, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `yzw`.
        #[inline] pub fn yzw(&self) -> $vector_type3<$S> { $vector_type3::new(self.y, self.z, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `zzw`.
        #[inline] pub fn zzw(&self) -> $vector_type3<$S> { $vector_type3::new(self.z, self.z, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `wzw`.
        #[inline] pub fn wzw(&self) -> $vector_type3<$S> { $vector_type3::new(self.w, self.z, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `xww`.
        #[inline] pub fn xww(&self) -> $vector_type3<$S> { $vector_type3::new(self.x, self.w, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `yww`.
        #[inline] pub fn yww(&self) -> $vector_type3<$S> { $vector_type3::new(self.y, self.w, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `zww`.
        #[inline] pub fn zww(&self) -> $vector_type3<$S> { $vector_type3::new(self.z, self.w, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 3 from variables `www`.
        #[inline] pub fn www(&self) -> $vector_type3<$S> { $vector_type3::new(self.w, self.w, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xxxx`.
        #[inline] pub fn xxxx(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.x, self.x, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yxxx`.
        #[inline] pub fn yxxx(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.x, self.x, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zxxx`.
        #[inline] pub fn zxxx(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.x, self.x, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wxxx`.
        #[inline] pub fn wxxx(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.x, self.x, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xyxx`.
        #[inline] pub fn xyxx(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.y, self.x, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yyxx`.
        #[inline] pub fn yyxx(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.y, self.x, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zyxx`.
        #[inline] pub fn zyxx(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.y, self.x, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wyxx`.
        #[inline] pub fn wyxx(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.y, self.x, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xzxx`.
        #[inline] pub fn xzxx(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.z, self.x, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yzxx`.
        #[inline] pub fn yzxx(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.z, self.x, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zzxx`.
        #[inline] pub fn zzxx(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.z, self.x, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wzxx`.
        #[inline] pub fn wzxx(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.z, self.x, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xwxx`.
        #[inline] pub fn xwxx(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.w, self.x, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `ywxx`.
        #[inline] pub fn ywxx(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.w, self.x, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zwxx`.
        #[inline] pub fn zwxx(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.w, self.x, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wwxx`.
        #[inline] pub fn wwxx(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.w, self.x, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xxyx`.
        #[inline] pub fn xxyx(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.x, self.y, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yxyx`.
        #[inline] pub fn yxyx(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.x, self.y, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zxyx`.
        #[inline] pub fn zxyx(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.x, self.y, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wxyx`.
        #[inline] pub fn wxyx(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.x, self.y, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xyyx`.
        #[inline] pub fn xyyx(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.y, self.y, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yyyx`.
        #[inline] pub fn yyyx(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.y, self.y, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zyyx`.
        #[inline] pub fn zyyx(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.y, self.y, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wyyx`.
        #[inline] pub fn wyyx(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.y, self.y, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xzyx`.
        #[inline] pub fn xzyx(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.z, self.y, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yzyx`.
        #[inline] pub fn yzyx(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.z, self.y, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zzyx`.
        #[inline] pub fn zzyx(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.z, self.y, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wzyx`.
        #[inline] pub fn wzyx(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.z, self.y, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xwyx`.
        #[inline] pub fn xwyx(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.w, self.y, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `ywyx`.
        #[inline] pub fn ywyx(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.w, self.y, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zwyx`.
        #[inline] pub fn zwyx(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.w, self.y, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wwyx`.
        #[inline] pub fn wwyx(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.w, self.y, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xxzx`.
        #[inline] pub fn xxzx(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.x, self.z, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yxzx`.
        #[inline] pub fn yxzx(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.x, self.z, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zxzx`.
        #[inline] pub fn zxzx(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.x, self.z, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wxzx`.
        #[inline] pub fn wxzx(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.x, self.z, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xyzx`.
        #[inline] pub fn xyzx(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.y, self.z, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yyzx`.
        #[inline] pub fn yyzx(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.y, self.z, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zyzx`.
        #[inline] pub fn zyzx(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.y, self.z, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wyzx`.
        #[inline] pub fn wyzx(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.y, self.z, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xzzx`.
        #[inline] pub fn xzzx(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.z, self.z, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yzzx`.
        #[inline] pub fn yzzx(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.z, self.z, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zzzx`.
        #[inline] pub fn zzzx(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.z, self.z, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wzzx`.
        #[inline] pub fn wzzx(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.z, self.z, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xwzx`.
        #[inline] pub fn xwzx(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.w, self.z, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `ywzx`.
        #[inline] pub fn ywzx(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.w, self.z, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zwzx`.
        #[inline] pub fn zwzx(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.w, self.z, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wwzx`.
        #[inline] pub fn wwzx(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.w, self.z, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xxwx`.
        #[inline] pub fn xxwx(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.x, self.w, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yxwx`.
        #[inline] pub fn yxwx(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.x, self.w, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zxwx`.
        #[inline] pub fn zxwx(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.x, self.w, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wxwx`.
        #[inline] pub fn wxwx(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.x, self.w, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xywx`.
        #[inline] pub fn xywx(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.y, self.w, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yywx`.
        #[inline] pub fn yywx(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.y, self.w, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zywx`.
        #[inline] pub fn zywx(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.y, self.w, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wywx`.
        #[inline] pub fn wywx(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.y, self.w, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xzwx`.
        #[inline] pub fn xzwx(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.z, self.w, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yzwx`.
        #[inline] pub fn yzwx(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.z, self.w, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zzwx`.
        #[inline] pub fn zzwx(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.z, self.w, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wzwx`.
        #[inline] pub fn wzwx(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.z, self.w, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xwwx`.
        #[inline] pub fn xwwx(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.w, self.w, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `ywwx`.
        #[inline] pub fn ywwx(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.w, self.w, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zwwx`.
        #[inline] pub fn zwwx(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.w, self.w, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wwwx`.
        #[inline] pub fn wwwx(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.w, self.w, self.x, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xxxy`.
        #[inline] pub fn xxxy(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.x, self.x, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yxxy`.
        #[inline] pub fn yxxy(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.x, self.x, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zxxy`.
        #[inline] pub fn zxxy(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.x, self.x, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wxxy`.
        #[inline] pub fn wxxy(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.x, self.x, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xyxy`.
        #[inline] pub fn xyxy(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.y, self.x, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yyxy`.
        #[inline] pub fn yyxy(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.y, self.x, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zyxy`.
        #[inline] pub fn zyxy(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.y, self.x, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wyxy`.
        #[inline] pub fn wyxy(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.y, self.x, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xzxy`.
        #[inline] pub fn xzxy(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.z, self.x, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yzxy`.
        #[inline] pub fn yzxy(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.z, self.x, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zzxy`.
        #[inline] pub fn zzxy(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.z, self.x, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wzxy`.
        #[inline] pub fn wzxy(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.z, self.x, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xwxy`.
        #[inline] pub fn xwxy(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.w, self.x, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `ywxy`.
        #[inline] pub fn ywxy(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.w, self.x, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zwxy`.
        #[inline] pub fn zwxy(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.w, self.x, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wwxy`.
        #[inline] pub fn wwxy(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.w, self.x, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xxyy`.
        #[inline] pub fn xxyy(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.x, self.y, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yxyy`.
        #[inline] pub fn yxyy(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.x, self.y, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zxyy`.
        #[inline] pub fn zxyy(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.x, self.y, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wxyy`.
        #[inline] pub fn wxyy(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.x, self.y, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xyyy`.
        #[inline] pub fn xyyy(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.y, self.y, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yyyy`.
        #[inline] pub fn yyyy(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.y, self.y, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zyyy`.
        #[inline] pub fn zyyy(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.y, self.y, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wyyy`.
        #[inline] pub fn wyyy(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.y, self.y, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xzyy`.
        #[inline] pub fn xzyy(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.z, self.y, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yzyy`.
        #[inline] pub fn yzyy(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.z, self.y, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zzyy`.
        #[inline] pub fn zzyy(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.z, self.y, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wzyy`.
        #[inline] pub fn wzyy(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.z, self.y, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xwyy`.
        #[inline] pub fn xwyy(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.w, self.y, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `ywyy`.
        #[inline] pub fn ywyy(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.w, self.y, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zwyy`.
        #[inline] pub fn zwyy(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.w, self.y, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wwyy`.
        #[inline] pub fn wwyy(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.w, self.y, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xxzy`.
        #[inline] pub fn xxzy(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.x, self.z, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yxzy`.
        #[inline] pub fn yxzy(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.x, self.z, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zxzy`.
        #[inline] pub fn zxzy(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.x, self.z, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wxzy`.
        #[inline] pub fn wxzy(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.x, self.z, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xyzy`.
        #[inline] pub fn xyzy(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.y, self.z, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yyzy`.
        #[inline] pub fn yyzy(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.y, self.z, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zyzy`.
        #[inline] pub fn zyzy(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.y, self.z, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wyzy`.
        #[inline] pub fn wyzy(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.y, self.z, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xzzy`.
        #[inline] pub fn xzzy(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.z, self.z, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yzzy`.
        #[inline] pub fn yzzy(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.z, self.z, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zzzy`.
        #[inline] pub fn zzzy(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.z, self.z, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wzzy`.
        #[inline] pub fn wzzy(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.z, self.z, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xwzy`.
        #[inline] pub fn xwzy(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.w, self.z, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `ywzy`.
        #[inline] pub fn ywzy(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.w, self.z, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zwzy`.
        #[inline] pub fn zwzy(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.w, self.z, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wwzy`.
        #[inline] pub fn wwzy(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.w, self.z, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xxwy`.
        #[inline] pub fn xxwy(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.x, self.w, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yxwy`.
        #[inline] pub fn yxwy(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.x, self.w, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zxwy`.
        #[inline] pub fn zxwy(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.x, self.w, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wxwy`.
        #[inline] pub fn wxwy(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.x, self.w, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xywy`.
        #[inline] pub fn xywy(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.y, self.w, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yywy`.
        #[inline] pub fn yywy(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.y, self.w, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zywy`.
        #[inline] pub fn zywy(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.y, self.w, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wywy`.
        #[inline] pub fn wywy(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.y, self.w, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xzwy`.
        #[inline] pub fn xzwy(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.z, self.w, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yzwy`.
        #[inline] pub fn yzwy(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.z, self.w, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zzwy`.
        #[inline] pub fn zzwy(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.z, self.w, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wzwy`.
        #[inline] pub fn wzwy(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.z, self.w, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xwwy`.
        #[inline] pub fn xwwy(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.w, self.w, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `ywwy`.
        #[inline] pub fn ywwy(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.w, self.w, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zwwy`.
        #[inline] pub fn zwwy(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.w, self.w, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wwwy`.
        #[inline] pub fn wwwy(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.w, self.w, self.y, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xxxz`.
        #[inline] pub fn xxxz(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.x, self.x, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yxxz`.
        #[inline] pub fn yxxz(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.x, self.x, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zxxz`.
        #[inline] pub fn zxxz(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.x, self.x, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wxxz`.
        #[inline] pub fn wxxz(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.x, self.x, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xyxz`.
        #[inline] pub fn xyxz(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.y, self.x, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yyxz`.
        #[inline] pub fn yyxz(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.y, self.x, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zyxz`.
        #[inline] pub fn zyxz(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.y, self.x, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wyxz`.
        #[inline] pub fn wyxz(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.y, self.x, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xzxz`.
        #[inline] pub fn xzxz(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.z, self.x, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yzxz`.
        #[inline] pub fn yzxz(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.z, self.x, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zzxz`.
        #[inline] pub fn zzxz(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.z, self.x, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wzxz`.
        #[inline] pub fn wzxz(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.z, self.x, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xwxz`.
        #[inline] pub fn xwxz(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.w, self.x, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `ywxz`.
        #[inline] pub fn ywxz(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.w, self.x, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zwxz`.
        #[inline] pub fn zwxz(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.w, self.x, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wwxz`.
        #[inline] pub fn wwxz(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.w, self.x, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xxyz`.
        #[inline] pub fn xxyz(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.x, self.y, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yxyz`.
        #[inline] pub fn yxyz(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.x, self.y, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zxyz`.
        #[inline] pub fn zxyz(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.x, self.y, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wxyz`.
        #[inline] pub fn wxyz(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.x, self.y, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xyyz`.
        #[inline] pub fn xyyz(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.y, self.y, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yyyz`.
        #[inline] pub fn yyyz(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.y, self.y, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zyyz`.
        #[inline] pub fn zyyz(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.y, self.y, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wyyz`.
        #[inline] pub fn wyyz(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.y, self.y, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xzyz`.
        #[inline] pub fn xzyz(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.z, self.y, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yzyz`.
        #[inline] pub fn yzyz(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.z, self.y, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zzyz`.
        #[inline] pub fn zzyz(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.z, self.y, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wzyz`.
        #[inline] pub fn wzyz(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.z, self.y, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xwyz`.
        #[inline] pub fn xwyz(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.w, self.y, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `ywyz`.
        #[inline] pub fn ywyz(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.w, self.y, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zwyz`.
        #[inline] pub fn zwyz(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.w, self.y, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wwyz`.
        #[inline] pub fn wwyz(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.w, self.y, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xxzz`.
        #[inline] pub fn xxzz(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.x, self.z, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yxzz`.
        #[inline] pub fn yxzz(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.x, self.z, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zxzz`.
        #[inline] pub fn zxzz(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.x, self.z, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wxzz`.
        #[inline] pub fn wxzz(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.x, self.z, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xyzz`.
        #[inline] pub fn xyzz(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.y, self.z, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yyzz`.
        #[inline] pub fn yyzz(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.y, self.z, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zyzz`.
        #[inline] pub fn zyzz(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.y, self.z, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wyzz`.
        #[inline] pub fn wyzz(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.y, self.z, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xzzz`.
        #[inline] pub fn xzzz(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.z, self.z, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yzzz`.
        #[inline] pub fn yzzz(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.z, self.z, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zzzz`.
        #[inline] pub fn zzzz(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.z, self.z, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wzzz`.
        #[inline] pub fn wzzz(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.z, self.z, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xwzz`.
        #[inline] pub fn xwzz(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.w, self.z, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `ywzz`.
        #[inline] pub fn ywzz(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.w, self.z, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zwzz`.
        #[inline] pub fn zwzz(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.w, self.z, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wwzz`.
        #[inline] pub fn wwzz(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.w, self.z, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xxwz`.
        #[inline] pub fn xxwz(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.x, self.w, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yxwz`.
        #[inline] pub fn yxwz(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.x, self.w, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zxwz`.
        #[inline] pub fn zxwz(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.x, self.w, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wxwz`.
        #[inline] pub fn wxwz(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.x, self.w, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xywz`.
        #[inline] pub fn xywz(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.y, self.w, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yywz`.
        #[inline] pub fn yywz(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.y, self.w, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zywz`.
        #[inline] pub fn zywz(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.y, self.w, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wywz`.
        #[inline] pub fn wywz(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.y, self.w, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xzwz`.
        #[inline] pub fn xzwz(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.z, self.w, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yzwz`.
        #[inline] pub fn yzwz(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.z, self.w, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zzwz`.
        #[inline] pub fn zzwz(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.z, self.w, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wzwz`.
        #[inline] pub fn wzwz(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.z, self.w, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xwwz`.
        #[inline] pub fn xwwz(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.w, self.w, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `ywwz`.
        #[inline] pub fn ywwz(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.w, self.w, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zwwz`.
        #[inline] pub fn zwwz(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.w, self.w, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wwwz`.
        #[inline] pub fn wwwz(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.w, self.w, self.z, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xxxw`.
        #[inline] pub fn xxxw(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.x, self.x, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yxxw`.
        #[inline] pub fn yxxw(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.x, self.x, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zxxw`.
        #[inline] pub fn zxxw(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.x, self.x, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wxxw`.
        #[inline] pub fn wxxw(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.x, self.x, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xyxw`.
        #[inline] pub fn xyxw(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.y, self.x, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yyxw`.
        #[inline] pub fn yyxw(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.y, self.x, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zyxw`.
        #[inline] pub fn zyxw(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.y, self.x, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wyxw`.
        #[inline] pub fn wyxw(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.y, self.x, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xzxw`.
        #[inline] pub fn xzxw(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.z, self.x, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yzxw`.
        #[inline] pub fn yzxw(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.z, self.x, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zzxw`.
        #[inline] pub fn zzxw(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.z, self.x, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wzxw`.
        #[inline] pub fn wzxw(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.z, self.x, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xwxw`.
        #[inline] pub fn xwxw(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.w, self.x, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `ywxw`.
        #[inline] pub fn ywxw(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.w, self.x, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zwxw`.
        #[inline] pub fn zwxw(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.w, self.x, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wwxw`.
        #[inline] pub fn wwxw(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.w, self.x, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xxyw`.
        #[inline] pub fn xxyw(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.x, self.y, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yxyw`.
        #[inline] pub fn yxyw(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.x, self.y, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zxyw`.
        #[inline] pub fn zxyw(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.x, self.y, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wxyw`.
        #[inline] pub fn wxyw(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.x, self.y, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xyyw`.
        #[inline] pub fn xyyw(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.y, self.y, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yyyw`.
        #[inline] pub fn yyyw(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.y, self.y, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zyyw`.
        #[inline] pub fn zyyw(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.y, self.y, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wyyw`.
        #[inline] pub fn wyyw(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.y, self.y, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xzyw`.
        #[inline] pub fn xzyw(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.z, self.y, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yzyw`.
        #[inline] pub fn yzyw(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.z, self.y, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zzyw`.
        #[inline] pub fn zzyw(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.z, self.y, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wzyw`.
        #[inline] pub fn wzyw(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.z, self.y, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xwyw`.
        #[inline] pub fn xwyw(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.w, self.y, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `ywyw`.
        #[inline] pub fn ywyw(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.w, self.y, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zwyw`.
        #[inline] pub fn zwyw(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.w, self.y, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wwyw`.
        #[inline] pub fn wwyw(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.w, self.y, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xxzw`.
        #[inline] pub fn xxzw(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.x, self.z, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yxzw`.
        #[inline] pub fn yxzw(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.x, self.z, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zxzw`.
        #[inline] pub fn zxzw(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.x, self.z, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wxzw`.
        #[inline] pub fn wxzw(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.x, self.z, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xyzw`.
        #[inline] pub fn xyzw(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.y, self.z, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yyzw`.
        #[inline] pub fn yyzw(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.y, self.z, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zyzw`.
        #[inline] pub fn zyzw(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.y, self.z, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wyzw`.
        #[inline] pub fn wyzw(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.y, self.z, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xzzw`.
        #[inline] pub fn xzzw(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.z, self.z, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yzzw`.
        #[inline] pub fn yzzw(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.z, self.z, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zzzw`.
        #[inline] pub fn zzzw(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.z, self.z, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wzzw`.
        #[inline] pub fn wzzw(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.z, self.z, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xwzw`.
        #[inline] pub fn xwzw(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.w, self.z, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `ywzw`.
        #[inline] pub fn ywzw(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.w, self.z, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zwzw`.
        #[inline] pub fn zwzw(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.w, self.z, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wwzw`.
        #[inline] pub fn wwzw(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.w, self.z, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xxww`.
        #[inline] pub fn xxww(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.x, self.w, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yxww`.
        #[inline] pub fn yxww(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.x, self.w, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zxww`.
        #[inline] pub fn zxww(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.x, self.w, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wxww`.
        #[inline] pub fn wxww(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.x, self.w, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xyww`.
        #[inline] pub fn xyww(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.y, self.w, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yyww`.
        #[inline] pub fn yyww(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.y, self.w, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zyww`.
        #[inline] pub fn zyww(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.y, self.w, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wyww`.
        #[inline] pub fn wyww(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.y, self.w, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xzww`.
        #[inline] pub fn xzww(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.z, self.w, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `yzww`.
        #[inline] pub fn yzww(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.z, self.w, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zzww`.
        #[inline] pub fn zzww(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.z, self.w, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wzww`.
        #[inline] pub fn wzww(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.z, self.w, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `xwww`.
        #[inline] pub fn xwww(&self) -> $vector_type4<$S> { $vector_type4::new(self.x, self.w, self.w, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `ywww`.
        #[inline] pub fn ywww(&self) -> $vector_type4<$S> { $vector_type4::new(self.y, self.w, self.w, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `zwww`.
        #[inline] pub fn zwww(&self) -> $vector_type4<$S> { $vector_type4::new(self.z, self.w, self.w, self.w, ) }

        /// Swizzle operator that creates a new type with dimension 4 from variables `wwww`.
        #[inline] pub fn wwww(&self) -> $vector_type4<$S> { $vector_type4::new(self.w, self.w, self.w, self.w, ) }

    };
}