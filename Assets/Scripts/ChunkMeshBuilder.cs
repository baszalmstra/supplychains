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

            // Compute occlusion
            var occlusion = ComputeFaceAverageOcclusion(chunk, localPos, face);

            // Add all vertices
            for (int i = 0; i < 4; ++i)
            {
                int vertex = Faces[(int)face, i];
                mesh.AddVertex(Vertices[vertex] * Constants.BlockSize.x + new Vector3(
                    localPos.x * Constants.BlockSize.x,
                    localPos.y * Constants.BlockSize.y,
                    localPos.z * Constants.BlockSize.z), 
                    normal, 
                    color,
                    new Vector2(occlusion[i], 0.0f));
            }

            mesh.AddQuad(occlusion[0] + occlusion[2] < occlusion[1] + occlusion[3]);
        }
    }

    static float[] ComputeFaceAverageOcclusion(Chunk chunk, BlockPos localPos, BlockFace face)
    {
        float[] result = new float[4];
        for(int vertexIndex = 0; vertexIndex < 4; ++vertexIndex)
        {
            float occlusion = 0.0f;
            for(int occlusionIndex = 0; occlusionIndex < 4; ++occlusionIndex)
            {
                var neighorPos = localPos + AdjacencyIndices[(int)face,vertexIndex,occlusionIndex];
                var neighbor = chunk.LocalGet(neighorPos);
                occlusion += neighbor.GetLightTransparency();
            }
            result[vertexIndex] = occlusion / 4.0f;
        }

        return result;
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

    static readonly BlockPos[,,] AdjacencyIndices = new BlockPos[,,] {
        // Back
        {
            { new BlockPos(-1,  0,  1), new BlockPos(-1,  1,  1), new BlockPos( 0,  0,  1), new BlockPos( 0,  1,  1) }, // 7
            { new BlockPos( 0,  0,  1), new BlockPos( 0,  1,  1), new BlockPos( 1,  0,  1), new BlockPos( 1,  1,  1) }, // 6
            { new BlockPos(0, -1,  1), new BlockPos(0,  0,  1), new BlockPos(1, -1,  1), new BlockPos(1,  0,  1)     }, // 2
            { new BlockPos(-1, -1,  1), new BlockPos(-1,  0,  1), new BlockPos( 0, -1,  1), new BlockPos( 0,  0,  1) }, // 3
        },

        // Right
        {
            { new BlockPos(1,  0,  0), new BlockPos(1,  0,  1), new BlockPos(1,  1,  1), new BlockPos(1,  1,  0) }, // 6
            { new BlockPos(1,  0, -1), new BlockPos(1,  0,  0), new BlockPos(1,  1,  0), new BlockPos(1,  1, -1) }, // 5
            { new BlockPos(1, -1, -1), new BlockPos(1, -1,  0), new BlockPos(1,  0,  0), new BlockPos(1,  0, -1) }, // 1
            { new BlockPos(1, -1,  0), new BlockPos(1, -1,  1), new BlockPos(1,  0,  1), new BlockPos(1,  0,  0) }, // 2
        },

        // Front
        {
            { new BlockPos( 0,  0, -1), new BlockPos( 0,  1, -1), new BlockPos( 1,  0, -1), new BlockPos( 1,  1, -1) }, // 5
            { new BlockPos(-1,  0, -1), new BlockPos(-1,  1, -1), new BlockPos( 0,  0, -1), new BlockPos( 0,  1, -1) }, // 4
            { new BlockPos(-1, -1, -1), new BlockPos(-1,  0, -1), new BlockPos( 0, -1, -1), new BlockPos( 0,  0, -1) }, // 0
            { new BlockPos(0, -1, -1), new BlockPos(0,  0, -1), new BlockPos(1, -1, -1), new BlockPos(1,  0, -1) }, // 1
        },

        // Left
        {
            { new BlockPos(-1,  0, -1), new BlockPos(-1,  0,  0), new BlockPos(-1,  1,  0), new BlockPos(-1,  1, -1) }, // 4
            { new BlockPos(-1,  0,  0), new BlockPos(-1,  0,  1), new BlockPos(-1,  1,  1), new BlockPos(-1,  1,  0) }, // 7
            { new BlockPos(-1, -1,  0), new BlockPos(-1, -1,  1), new BlockPos(-1,  0,  1), new BlockPos(-1,  0,  0) }, // 3
            { new BlockPos(-1, -1, -1), new BlockPos(-1, -1,  0), new BlockPos(-1,  0,  0), new BlockPos(-1,  0, -1) }, // 0
        },      

        // Top
        {
            { new BlockPos( 0,  1,  0), new BlockPos( 0,  1,  1), new BlockPos( 1,  1,  0), new BlockPos( 1,  1,  1) }, // 6
            { new BlockPos(-1,  1,  0), new BlockPos(-1,  1,  1), new BlockPos( 0,  1,  0), new BlockPos( 0,  1,  1) }, // 7
            { new BlockPos(-1,  1, -1), new BlockPos(-1,  1,  0), new BlockPos( 0,  1, -1), new BlockPos( 0,  1,  0) }, // 4
            { new BlockPos( 0,  1, -1), new BlockPos( 0,  1,  0), new BlockPos( 1,  1, -1), new BlockPos( 1,  1,  0) }, // 5
        },

        // Bottom
        {
            { new BlockPos( 0, -1, -1), new BlockPos( 0, -1,  0), new BlockPos( 1, -1, -1), new BlockPos( 1, -1,  0) }, // 1
            { new BlockPos(-1, -1, -1), new BlockPos(-1, -1,  0), new BlockPos( 0, -1, -1), new BlockPos( 0, -1,  0) }, // 0
            { new BlockPos(-1, -1,  0), new BlockPos(-1, -1,  1), new BlockPos( 0, -1,  0), new BlockPos( 0, -1,  1) }, // 3
            { new BlockPos( 0, -1,  0), new BlockPos( 0, -1,  1), new BlockPos( 1, -1,  0), new BlockPos( 1, -1,  1) } // 2
        },
    };
}
