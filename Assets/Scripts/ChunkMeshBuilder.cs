using UnityEngine;
using System.Collections;
using System;

public static class ChunkMeshBuilder
{
    public static Mesh Build(Chunk chunk)
    {
        MeshBuilder mesh = new MeshBuilder();

        for(int x = 0; x < Constants.ChunkSize; ++x)
            for(int y = 0; y < Constants.ChunkLayers; ++y)
                for(int z = 0; z < Constants.ChunkSize; ++z)
                {
                    BlockPos localPos = new BlockPos(x,y,z);
                    if(chunk.LocalGet(localPos) >= Block.Grass)
                        BuildBlock(localPos, chunk, mesh);
                }
        
        return mesh.Build();
    }

    static void BuildBlock(BlockPos localPos, Chunk chunk, MeshBuilder mesh)
    {
        float shade = (localPos + chunk.Position).Random(0, true) * 0.2f + 0.8f;
        foreach (BlockFace face in (BlockFace[])Enum.GetValues(typeof(BlockFace)))
        {
            if (!chunk.LocalGet(localPos + face).IsTransparent())
                continue;

            // Get the color
            Color color = Color.white * shade;
            color.a = 1.0f;

            // Get the face normal
            Vector3 normal = FaceNormals[(int)face];

            // Add all vertices
            for (int i = 0; i < 4; ++i)
            {
                int vertex = Faces[(int)face, i];
                mesh.AddVertex(Vertices[vertex] * Constants.BlockSize.x + new Vector3(
                    localPos.x * Constants.BlockSize.x,
                    localPos.y * Constants.BlockSize.y,
                    localPos.z * Constants.BlockSize.z), normal, color);
            }

            mesh.AddQuad();
        }
    }

    static readonly int[,] Faces = new int[,] {
        { 7, 6, 2, 3 },
        { 6, 5, 1, 2 },
        { 5, 4, 0, 1 },
        { 4, 7, 3, 0 },
        { 6, 7, 4, 5 },
        { 1, 0, 3, 2 }
    };

    static readonly Vector3[] FaceNormals = new Vector3[] {
        new Vector3( 0f,  0f,  1f),
        new Vector3( 1f,  0f,  0f),
        new Vector3( 0f,  0f, -1f),
        new Vector3(-1f,  0f,  0f),        
        new Vector3( 0f,  1f,  0f),
        new Vector3( 0f, -1f,  0f)
    };

    static readonly Vector3[] Vertices = new Vector3[] {
        new Vector3(0f, 0f, 0f),
        new Vector3(1f, 0f, 0f),
        new Vector3(1f, 0f, 1f),
        new Vector3(0f, 0f, 1f),
        new Vector3(0f, 1f, 0f),
        new Vector3(1f, 1f, 0f),
        new Vector3(1f, 1f, 1f),
        new Vector3(0f, 1f, 1f),
    };
}
