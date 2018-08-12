using System;
using System.Collections.Generic;
using System.IO;
using Ur;

namespace StbSharp.StbImage.Generator
{
	class Program
	{
		private const string SourceFile = @"..\..\..\..\..\src\stb_image.rs";

		static void Process()
		{
			var skipFunctions = new HashSet<string>
			{
/*				"stbi__malloc",
				"stbi_image_free",
				"stbi_is_hdr_from_memory",
				"stbi_is_hdr_from_callbacks",
				"stbi__pnm_isspace",
				"stbi__pnm_skip_whitespace",
				"stbi__pic_is4",
				"stbi__gif_parse_colortable"*/
				"stbi_failure_reason",
				"stbi__err",
				"stbi__tga_test"
			};

			var skipStructs = new HashSet<string>
			{
				"stbi_io_callbacks",
				"stbi__resample",
				"stbi__jpeg",
/*				"img_comp",
				"stbi__resample",
				"stbi__gif_lzw",
				"stbi__gif"*/
			};

			var skipGlobalVariables = new HashSet<string>
			{
				"stbi__g_failure_reason",
//				"stbi__vertically_flip_on_load"
			};

			var parameters = new ConversionParameters
			{
				InputPath = @"stb_image.h",
				Defines = new[]
				{
						"STBI_NO_SIMD",
						"STBI_NO_LINEAR",
						"STBI_NO_HDR",
						"STBI_NO_PIC",
						"STBI_NO_PNM",
						"STBI_NO_STDIO",
						"STB_IMAGE_IMPLEMENTATION",
						"STBI_ONLY_PNG"
					},
				AddGeneratedByUr = true
			};

			parameters.StructSource = n =>
			{
				var result = new BaseConfig
				{
					Name = n,
					Source = SourceFile
				};

				if (skipStructs.Contains(n))
				{
					result.Source = null;
				}

				return result;
			};

			parameters.GlobalVariableSource = n =>
			{
				var result = new BaseConfig
				{
					Name = n,
					Source = SourceFile,
				};

				if (skipGlobalVariables.Contains(n))
				{
					result.Source = null;
				}

				return result;
			};

			parameters.EnumSource = n => new BaseConfig
			{
				Name = string.Empty,
				Source = SourceFile
			};


			parameters.FunctionSource = n =>
			{
				var fc = new BaseConfig
				{
					Name = n.Name,
					Source = SourceFile,
				};

				if (skipFunctions.Contains(n.Name))
				{
					fc.Source = null;
				}

				return fc;
			};

			var cp = new ClangParser();

			var outputs = cp.Process(parameters);

			foreach (var output in outputs)
			{
				var data = output.Value;

				// Post processing
				Logger.Info("Post processing...");

				data = Utility.ReplaceNativeCalls(data);

				data = data.Replace("pub static mut __security_cookie:u64 = std::mem::uninitialized();", "");
				data = data.Replace("stbi__zdist_extra:[i32;32]", "stbi__zdist_extra:[i32;30]");
				data = data.Replace(",;", "");
				data = data.Replace("std::mem::size_of(s.buffer_start)", "s.buffer_start.len()");
				data = data.Replace("(0) as *mut u8", "std::ptr::null_mut()");
				data = data.Replace("\".as_mut_ptr()", "\"");
				data = data.Replace("io.read = (std::ptr::null_mut())", "io.read = None");
				data = data.Replace("(std::mem::size_of((*s).buffer_start))", "(*s).buffer_start.len()");
				data = data.Replace("std::mem::size_of(sizes)", "sizes.len()");
				data = data.Replace("std::mem::size_of((*z).fast)", "(*z).fast.len()");
				data = data.Replace("std::mem::size_of(codelength_sizes)", "codelength_sizes.len()");
				data = data.Replace("\") as *mut i8", "\")");
				data = data.Replace("if ((*s).io.read) != std::ptr::null_mut()", "if ((*s).io.read.is_some())");
				data = data.Replace("((*s).io.read)", "((*s).io.read.unwrap())");
				data = data.Replace("((dist) as i64)", "dist as usize");
				data = data.Replace("(&mut raw_len) as *mut i32", "(&mut raw_len) as *mut u32");
				data = data.Replace("(*a).s.", "(*(*a).s).");
				data = data.Replace("(*p).s.", "(*(*p).s).");
				data = data.Replace("result = stbi__convert_8_to_16(((result) as *mut u8), ((*x) as i32), ((*y) as i32), ((cmp) as i32))",
					"result = stbi__convert_8_to_16(((result) as *mut u8), ((*x) as i32), ((*y) as i32), ((cmp) as i32)) as *mut u8");
				data = data.Replace("result = stbi__convert_format16((((result) as *mut u16) as *mut u16), (((*(*p).s).img_out_n) as i32), ((req_comp) as i32), (((*(*p).s).img_x) as u32), (((*(*p).s).img_y) as u32))",
					"result = stbi__convert_format16((((result) as *mut u16) as *mut u16), (((*(*p).s).img_out_n) as i32), ((req_comp) as i32), (((*(*p).s).img_x) as u32), (((*(*p).s).img_y) as u32)) as *mut u8");

				var manualData = File.ReadAllText("stb_image_man.rs");

				data += manualData;

				File.WriteAllText(output.Key, data);
			}
		}

		static void Main(string[] args)
		{
			try
			{
				Process();
			}
			catch (Exception ex)
			{
				Console.WriteLine(ex.ToString());
			}

			Console.WriteLine("Finished. Press any key to quit.");
			Console.ReadKey();
		}
	}
}