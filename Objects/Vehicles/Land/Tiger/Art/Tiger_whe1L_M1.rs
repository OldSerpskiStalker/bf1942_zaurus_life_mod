shader "Material0"
{
	technique
	{
		transparent true;
		pass
		{
			cullMode none;
			lighting true;
			lightingSpecular true;
			materialAmbient 1 1 1;
			materialDiffuse 0.5 0.5 0.5;
			materialSpecular 0 0 0;
			materialSpecularPower 12.5;
			alphaTest greater 0.7;

			stage
			{
				texture "texture/TiWhe1_z";
				combine color mul texture diffuse;
				combine alpha single texture;
			}
		}
	}
}

